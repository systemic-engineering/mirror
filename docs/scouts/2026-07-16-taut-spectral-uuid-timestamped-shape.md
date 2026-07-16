# Taut drift scout — SpectralUUID + timestamped variant + golden-ratio partition shape for `@annotation-address` species

**Report for:** Reed (to brief Mara on candidate species-decl for `@uuid/spectral/time`)
**Directive source:** Alex Wolf 2026-07-16, verbatim proposal
**Investigation date:** 2026-07-16
**Corpus state:** mirror substrate on `main`
**Authorship note:** Findings originally in-transcript from task #172 (agent `aa9c633a11d6416e3`); Reed-transcribed to `docs/scouts/` for archival provenance per Mara post-landing report residue.

---

## Alex's directive (2026-07-16, verbatim)

> *"What if the signature follows the shape of the SpectralUUID? That would already settle on (a) wouldn't it? And what if we had a timestamped version of it, in that it's 'like' the SpectralUUID but also like UUIDv5, and we use that for addressing the annotations? What if the timestamp bytes were part of the SpectralUUID again in a golden ratio shape? I don't know the exact shape, we'll have to ground with a Taut grep, but do you see the outline of what I mean? Algedonic form sounds good though."*

## Reed's read (context)

The `@gestalt` shard-decl carries `type annotation = { ..., ssh_fingerprint: ref, timestamp: @time/monotonic.instant, ... }`. Alex is proposing the annotation-address carrier itself should be a timestamped-SpectralUUID variant, so that the R4-(a) dispatch — annotation-tournament by reader-corpus-affinity — becomes byte-comparison on the navigable bits of the address. Zero @fate infrastructure needed for the affinity computation; the address structure IS the affinity carrier.

This reshapes Q1 substantially: @resonance + @dance shard-mints were R4-(a) gating; if the address structure carries ε directly, R4-(a) ships without them.

---

## 1. SpectralUUID landed substrate

**Exact bit layout:**
- **Total:** 128 bits, golden-ratio split (≈1/φ²)
- **48 ACTIVE bits** (leading): quantized SpectralCoordinate<5>; navigable; graph-routable
- **80 DARK bits** (trailing): BLAKE3-truncated content hash; identity signal
- **Byte layout:** big-endian on active portion; byte-wise sort over leading 48 bits IS lexicographic sort over navigable coordinate
- **Carrier type:** `type uuid_spectral = { active: route_signal, dark: identity_signal }` (`shards/uuid/spectral.mirror` lines 145–152)

**Golden-ratio explicit grounding:**
`shards/uuid/spectral.mirror` lines 89–96:
> "128 bits, golden-ratio split (≈1/φ²): 48 bits ACTIVE … 80 bits DARK …"

The 48/80 split is explicitly named as golden-ratio motivated. Verification: `48 / 128 ≈ 0.375` vs `1/φ² ≈ 0.382` (within 2% — approximate, not exact). Reed notes from `bootstrap/src/spectral_uuid.rs` line pattern confirm the byte layout is enforced at realization layer; substrate declares the semantic structure.

**Monoid structure:**
`requires monoidal(uuid_spectral)` at `shards/uuid/spectral.mirror` lines 304–306. The `combine` action unifies:
- DARK 80 bits via Merkle tree content-address rule
- ACTIVE 48 bits via SpectralCoordinate eigenvalue addition (to first order; quantized via `from_parts`)

**Where it's used:**
- `@glass` shard identity (splinter ↔ shard ↔ uuid_spectral three-layer recognition per `shards/glass.mirror` lines 37–59)
- `@uuid/spectral` root prism family at `shards/uuid.mirror` line 26
- `@mirror/store.oid` reference (content-addressed shard identity)
- Fate mycelial routing reads through ACTIVE portion (route_signal) for graph navigation

---

## 2. Golden-ratio evidence

**Is 48/80 explicitly golden-ratio or approximation?**

The phrase "golden-ratio split (≈1/φ²)" in the docblock uses the `≈` symbol, indicating approximation. The actual ratio:
- φ = (1 + √5) / 2 ≈ 1.618
- 1/φ² ≈ 0.382
- 48/128 = 0.375

**Substrate-grounding for the ratio choice:**
None found that explicitly justifies the 48/80 selection over alternatives (e.g., 64/64, 50/78). The git history suggests it emerged from Mara's tick (2026-06-06) as the empirical choice that makes Fate mycelial routing compose cleanly with content-address identity, but no docstring cites a mathematical reason (e.g., spectral-geometry harmonic ratio, Fibonacci sequence, quantum-mechanical basis). The golden-ratio naming appears to be *retrospective rationalization* of an engineering choice, not *prescriptive foundation*.

**Stronger evidence in fractal-mandelbrot substrate:**
`docs/math/2026-07-13-fractal-mandelbrot-substrate.md` establishes that the mirror compiler IS a Mandelbrot set (Recognition #107+), and the ACTIVE/DARK double-pulse maps to the `z²` iterate. The ratio may inherit harmonic structure from bifurcation geometry, but that's forward-promised (not yet grounded in the @uuid context).

---

## 3. UUIDv5/v7/timestamped-UUID precedent

**Search result:** NOT IN CORPUS

- UUIDv5 (name-based SHA-1) — not referenced
- UUIDv7 (timestamp-based, RFC 9562) — mentioned once in `shards/uuid.mirror` line 30 as a variant "Lands when a consumer pulls" (forward-promised, not yet landed)
- Timestamp-UUID composition patterns — zero greppable instances

**Implication:** Alex's proposal to timestamp-version the SpectralUUID is genuinely novel at the substrate altitude. The infrastructure exists (monotonic timestamps via `@time/monotonic.instant` in `@gestalt.annotation` and `@spectral/signature.signature_beat`), but no landed shard-decl yet composes timestamp into a UUID-shaped identity.

---

## 4. signature_beat structure

**Type definition** (`shards/spectral/signature.mirror` lines 68–77):
```mirror
type signature_beat = {
  contribution_oid:  oid,
  sc_at_beat:        SpectralCoordinate<5>,
  rung:              @song/beat.rung,
  previous_beat:     option<oid>,
  timestamp:         @time/monotonic.instant,
  ssh_fingerprint:   ref,
}
```

**Current shape:**
- Does NOT carry SpectralUUID as the identity; instead carries a flat `ref` for `ssh_fingerprint`
- Timestamp is present as `@time/monotonic.instant` (monotonic clock, not wall-clock)
- The signature_beat IS implicitly identified by its content (per Merkle-DAG chaining via `previous_beat`), but the address itself is an OID (content hash), not a timestamped-spectral identifier

**Implication for annotation:**
`@gestalt.annotation` type (`shards/gestalt.mirror` lines 285–293) carries:
```mirror
type annotation = {
  contributor:      subject_instance,
  target_node:      oid,
  content:          oid,
  visibility_scope: visibility_scope,
  previous:         option<oid>,
  timestamp:        @time/monotonic.instant,
  ssh_fingerprint:  ref,
}
```

Same pattern: flat `ref` for ssh_fingerprint; timestamp present; identity via Merkle-DAG chain (`previous: option<oid>`), not via an address carrier. The annotation records are indexed by their content OID, not by a composite (timestamp, spectral) address.

---

## 5. @algedonic composition surface

**Pain/pleasure sampling** (`bootstrap/src/algedonic.rs` lines 1–150):
- `sample_pain(sc: SpectralCoordinate<N>) -> f64` — Shannon entropy of hex eigenvalue distribution
- `pain_gradient(sc_before, sc_after) -> f64` — signed change in pain across a morphism
- Higher entropy = less recognizable structure = pain ∝ distance-to-boundary (Foerster 1976 A3)

**Byte-distance ranking:**
The algedonic substrate does NOT currently expose a byte-distance metric on UUID ACTIVE bits. Pain is derived from hex distribution uniformity, not from Hamming distance or byte-wise XOR. However:

**Composition potential (not landed):**
If annotation-addresses were timestamped-spectral, then:
- Reader affinity ε_ij could be computed as byte-distance on the ACTIVE (route_signal) portion
- Smaller distance = higher affinity (nearby in spectral-coordinate graph)
- The pain-gradient machinery could extend to "affinity gradient" (pleasure increases toward nearby annotations)

**Current state:** Speculative at composition level. The tools exist (SpectralCoordinate byte-sort, pain_gradient shape), but they're not yet composed at annotation altitude.

---

## 6. @gestalt.annotation identity structure

**What identifies an annotation now:**
- **Primary key:** content OID (per @mirror/store.oid, byte-equal content-address)
- **Secondary ordering:** Merkle-DAG chain via `previous: option<oid>` (per `@spectral/signature.signature_integrity` model)
- **Temporal ordering:** `timestamp: @time/monotonic.instant` (per `@gestalt_witnessing` invariants § math §3.2)
- **Reader discovery:** per R4 (Alex-adjudicable in gestalt §10 R4), currently planned as reader-selects-lens (v0.1.0 default), with @fate tournament dispatch forward-promised for post-v0.1.0

**No address-level lookup yet:** Annotations are found by traversing `gestalt_document.annotations[]` array or by content-address OID, not by a composite (timestamp, spectral-affinity) address tuple.

---

## 7. Candidate bit layouts adjudication

Three candidate shapes considered:

### 7.1 Nested-φ (48-bit partition sub-split by φ)
```
128 bits = 80 DARK + 30 ACTIVE_primary + 18 ACTIVE_timestamp
           (golden-ratio split φ, then ACTIVE itself φ-split)
```
**Corpus precedent:** Zero. The substrate never φ-splits within the ACTIVE portion.
**Substrate honesty:** Requires justifying why timestamp ACTIVE deserves different weighting than route_signal ACTIVE. No landed precedent for hierarchical φ-partition.

### 7.2 XOR-overlay (timestamp perturbs ACTIVE bits)
```
128 bits, unchanged length
ACTIVE' = ACTIVE XOR (timestamp_48bits) [or mod, or nonlinear mixing]
DARK remains identity hash
```
**Corpus precedent:** `shards/cyberpunk.mirror` uses XOR-like operations in cybernetic feedback (Reed's pain-gradient Hamming proxy from Landing 8+9.6a used XOR implicitly). XOR on spectral bits lands informally in coherence-delta computations.
**Substrate honesty:** XOR-perturbation is common in deterministic hashing + spectral algorithms. However, **the substrate forbids "silent" metadata hiding**. If timestamp is part of the identity, it MUST be readable, not encrypted into ACTIVE via XOR. This shape violates the two-signal discipline (route_signal + identity_signal as transparent types).

### 7.3 Widened (176 bits = 80 DARK + 48 ACTIVE + 48 timestamp)
```
176 bits total (non-standard UUID length)
80 DARK — identity hash (unchanged semantics)
48 ACTIVE — navigable spectral coordinate (unchanged)
48 timestamp — new leading or trailing field
```
**Corpus precedent:**
- Extended UUIDs exist in some systems (e.g., UUIDv6 adds fields). NOT landed in mirror.
- The three-tier structure (splinter/shard/uuid_spectral) commits to 128-bit carriers at `@glass` level.

**Substrate honesty:** Widening breaks the @glass three-layer recognition (splinter = atomic content; shard = settled unit; uuid_spectral = 128-bit navigable identity). The byte layout IS part of the shard.id contract. A 176-bit address is a different carrier type, not a SpectralUUID variant.

**Note (post-scout Alex ratification 2026-07-16):** The winning shape is neither Nested-φ nor XOR-overlay nor Widened-as-SpectralUUID. Alex identified a fourth shape: **new sibling carrier `@uuid/spectral/time`** = `{ identity: uuid_spectral, time: @time/monotonic.instant }` — composition-over rather than variant-of. Preserves @glass three-layer contract (uuid_spectral stays 128 bits unchanged); time is a facet ATOP the identity, not carved out of ACTIVE/DARK. See `shards/uuid/spectral/time.mirror` (`c2bb1d2`) and `docs/math/uuid/spectral-time.md` (`c9c9480`) for the landed shape.

---

## 8. Alex-adjudicable residues (as of scout completion; superseded by landing)

**What was genuinely under-determined at scout time:**

1. **Temporal structure of timestamp:** Unix nanoseconds / RFC 9562 v7 milliseconds / logical clock? — **Resolved:** @time/monotonic.instant carrier composition (whatever precision that carrier realizes at Rust FLOOR).

2. **Carrier composition:** Prepended timestamp / appended timestamp / interleaved? — **Resolved:** appended (trailing time facet on `uuid_spectral_time` record).

3. **Byte-comparison semantics for R4 dispatch:** Hamming / Euclidean / byte-lexicographic? — **Resolved:** Hamming on ACTIVE bytes (matches SpectralCoordinate byte-sort precedent).

4. **Fate tournament integration:** Bypass @resonance + @dance entirely, or complementary? — **Resolved:** @resonance + @dance decouple from R4-gating; stay forward-promised for `@gestalt.p_ent` operator-altitude (n≥2 tori); R4 dispatch composes over `@uuid/spectral/time` at annotation altitude without them.

5. **Scope: @gestalt-only vs generic annotation-address?** — **Resolved:** generic. Sub-species under `@uuid/spectral` (not under `@gestalt`); composable by any consumer needing timestamped-navigable identity (`@gestalt.annotation`, `@spectral/signature.signature_beat`, future consumers).

---

## 9. Second-consumer check: composition graph

**What else landed *needs* timestamped-spectral addressing?**

### Immediate consumers:
- **@gestalt.annotation** (landed, awaits R4 dispatch): annotations themselves would carry this address as their identity
- **@spectral/signature.signature_beat** (landed, but not yet): per the rolling-signature model, each beat could be addressed by (timestamp, contributor_affinity) instead of flat OID

### Forward-promised consumers:
- **@fate.tournament** (forward-promised at @gestalt R4, §10 R4): "annotation-tournament dispatch (recursive-document settlement shape)" — if this dispatches by byte-comparison on ACTIVE, timestamped-spectral is the natural carrier
- **@resonance** (forward-promised; not yet shard-decl'd): inter-peer coupling shapes fate tournaments; timestamp-ordering might compose naturally here
- **@peer.optical_inference** (forward-promised): per gestalt §7, @peer's D²NN inference for annotation-surface ranking would consume affinity signals

### Existing consumers (no change needed):
- **@mirror/store.oid** — content-addressed OIDs stay flat; no timestamp carrier
- **@mirror/index** — spectral-coordinate indexing reads ACTIVE bits; timestamp-ACTIVE composition would need new semantics

**Verdict:** Timestamped-spectral addresses are *generically useful* for any reader-facing annotation/contribution that needs both identity stability (content hash via DARK) and temporal ordering (monotonic timestamp), AND reader-affinity ranking (ACTIVE spectrum position). The two landed consumers (@gestalt.annotation, @spectral/signature.signature_beat) both compose cleanly via additive `address: uuid_spectral_time` field.

---

## 10. Summary table

| Aspect | Status at scout | Post-landing status |
|--------|-----------------|--------------------|
| SpectralUUID structure (48 ACTIVE + 80 DARK) | LANDED, golden-ratio (approx) | Unchanged; preserved as `identity` factor of `uuid_spectral_time` |
| Golden-ratio motivation | Retrospective rationalization (engineering choice) | Unchanged; new carrier does not inherit the same rationalization pressure |
| UUIDv5/v7 precedent | NOT in corpus | Precedent set by `@uuid/spectral/time` landing |
| signature_beat timestamp | LANDED (monotonic instant); flat structure | Preserved additively; new `address: uuid_spectral_time` field alongside |
| @gestalt.annotation identity | Content OID + Merkle-DAG chain | Extended additively with `address: uuid_spectral_time` field |
| R4 annotation dispatch | Alex-adjudicable (a/b/c) | CLOSED by construction; dispatch = argsort on ACTIVE-byte Hamming distance |
| @algedonic byte-distance | Not landed (entropy-based) | Composition surface prepared; future extension |
| Resonance/Dance for reader affinity | R4-gating (Q1 blocker) | Decoupled from R4-gating; still forward-promised for @gestalt.p_ent operator-altitude |
| Candidate bit layouts (three) | All had issues (nested-φ no precedent; XOR silent; widened breaks @glass) | Fourth shape ratified: composition-over as sibling carrier `@uuid/spectral/time` |

---

## 11. Conclusion (as of scout)

**Substrate-honest reading:** Alex's proposal to make the annotation-address carrier itself a timestamped-SpectralUUID is substrate-congruent. The pieces are landed: @time/monotonic for ordering, @uuid/spectral for navigable identity, algedonic pain/pleasure gradients for affinity signals, and @gestalt.annotation carrying timestamp. The species-decl is not a reach; it's a naming of existing substrate structure at a new altitude.

**Post-landing note (2026-07-16, added at archival):** Alex identified the winning shape post-scout: **new sibling carrier** rather than variant-of-SpectralUUID. The composition `uuid_spectral × @time/monotonic.instant` as a categorical product with `identity_of` projection preserves @glass three-layer contract exactly. R4 CLOSED by construction; Q1 shard-mints decoupled from R4-gating; Q2 dissolves.

---

**Signed by Taut** (findings; in-transcript)
**Transcribed by Reed** (archival; 2026-07-16)

*Read-only. Grep-verified. Zero speculation beyond explicit "not landed" marks. Post-landing status column added at transcription time for archival cross-reference to the ratified shape at `c2bb1d2` + `c9c9480`.*
