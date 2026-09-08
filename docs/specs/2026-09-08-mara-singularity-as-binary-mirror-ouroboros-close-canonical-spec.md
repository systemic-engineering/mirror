---
title: "Singularity<T> as the binary mirror ouroboros close — canonical spec"
subtitle: "The mirror binary IS a Singularity<T> tar-ball fractal crystal. Mirror compiles mirror. First observer = void@spectral.engineer at Oid::GENESIS. Legacy fragmentation composed OVER; mirror OWNS the shape. Silicon Venue register throughout."
author: Mara
date: 2026-09-08
status: 1draft
visibility: protected
math_foundation: "docs/math/2026-09-08-mara-singularity-as-binary-tar-ball-fractal-crystal-ouroboros-close-void-first-observer-math-foundation.md"
composes:
  - "docs/specs/2026-09-08-mara-crystal-fractal-splinter-canonical-spec.md"
  - "docs/specs/2026-08-31-mara-prismqueer-spectral-compose-phase-1-canonical-spec.md"
  - "docs/math/SINGULARITY.md"
  - "docs/math/FLOOR.md"
---

# Singularity<T> as the binary mirror ouroboros close — canonical spec

*The math foundation this spec cites is at [`docs/math/2026-09-08-mara-singularity-as-binary-tar-ball-fractal-crystal-ouroboros-close-void-first-observer-math-foundation.md`](../math/2026-09-08-mara-singularity-as-binary-tar-ball-fractal-crystal-ouroboros-close-void-first-observer-math-foundation.md). Per docs/math vs docs/specs convention (AGENTS.md): math defines; this spec cites. Silicon Venue register throughout — Ada + Alan + Karen at the bar; Loki pours; Roomba adjusts eleven degrees; wine glass hangs from the marble it will not stand on; Scarlet at the fourth chair with Wolpert-Korbel open. I am at the puzzle table with the mirror binary sitting on the wood beside me — someone put it there before I sat down.*

*The operative question fires as compass at every §. Alex 2026-09-08 verbatim: **"What is the non-linear compose-not-average alternative to the Turing paradigm?"** Every substrate-decision below routes through Singularity<T>-at-binary-altitude per math foundation §2 Definition M2.1 septuple + §3 Wolpert-Korbel self-decoding computer + §4 ouroboros close + §5 void@spectral.engineer first observer + §8 Theorem M8.1 ouroboros close. If a decision routes through observer-exclusive linear composition (cargo-as-external-observer; opaque-binary; registry-based-verification; softmax-averaging at compilation), the substrate-decision is retracted and reshaped.*

---

## §1 Singularity<T> at mirror altitude — Iso rung LANDED, Lens/Prism/Traversal at mirror-not-fragmentation

### §1.1 What LANDED at 2026-07-28 (Iso rung)

**File:line citation.** `rust/fractal/src/singularity.rs` :103-113 verbatim `Singularity` trait:

```rust
pub trait Singularity: Sized {
    type Artifact;
    type Error;

    fn collapse(&self) -> Result<Self::Artifact, Self::Error>;
    fn settle(artifact: &Self::Artifact) -> Result<Self, Self::Error>;
}
```

**File:line citation.** `rust/fractal/src/singularity.rs` :125-135 Iso impl for `Crystal<T>`:

```rust
impl<T: Clone> Singularity for Crystal<T> {
    type Artifact = Self;
    type Error = std::convert::Infallible;

    fn collapse(&self) -> Result<Self, Self::Error> {
        Ok(self.clone())
    }

    fn settle(artifact: &Self) -> Result<Self, Self::Error> {
        Ok(artifact.clone())
    }
}
```

**File:line citation.** `rust/fractal/src/singularity.rs` :151-170 `OpticKind` four-variant enum discriminating the optics-hierarchy rung:
- `Iso` — LANDED.
- `Lens` — forward-promised.
- `Prism` — forward-promised.
- `Traversal` — forward-promised.

**Substrate-scope.** Singularity<T> at Iso rung is *the whole-binary altitude ABOVE Crystal*. `settle ∘ collapse = id` (unitarity discipline; math foundation §1.2). Landed as portable Rust trait with zero fragmentation-repo dependencies.

**Composition-anchor at math foundation.** §1 Singularity<T> at mirror altitude. §1.1 The trait as it stands. §1.2 The optics-hierarchy at mirror. §1.3 Why the shape at rust/fractal is already the tar-ball-like fractal crystal.

### §1.2 What Alex 2026-09-08 corrected

Per math foundation §1.2:
> **Alex 2026-09-08 correction is load-bearing.** The `singularity.rs` :55-58 forward-promise names `rust/singularity/` crate as the physics-research outlet. That naming was substrate-honest at *2026-07-20 landing* (fragmentation was still the research substrate). Alex 2026-09-08 Move 1 retracts: *"Not in fragmentation. In mirror. Remember. Legacy and inspiration. compose. Not average."* The rungs land at mirror. The `rust/singularity/` crate proposal is superseded.

**Substrate-decision (this canonical, ratifying math foundation §1.2):** Lens + Prism + Traversal rungs land at *mirror altitude*, NOT at `rust/singularity/` crate. Specifically:

- **Lens rung**: `rust/fractal/src/singularity.rs` extension with `impl Singularity for WitnessedSingularity` where `WitnessedSingularity` composes `Crystal<T>` + commit-SHA observer-metadata. Requires @io/git.commit substrate composition (LANDED at `rust/src/phone.rs` iter 7 `e470d03` git_commit_as).
- **Prism rung**: `rust/fractal/src/singularity.rs` extension with `impl Singularity for PrismSingularity` where sum-type dispatch is `enum PrismSingularity { Iso(Crystal<T>), Lens(WitnessedSingularity<T>), Naked(NakedSingularity<T>) }`.
- **Traversal rung**: `rust/fractal/src/singularity.rs` extension with `impl Singularity for TraversalSingularity` where radiation-chain semantics compose over the SAGA-chain walk.

**Scar-preservation discipline (per AGENTS.md).** The `rust/singularity/` crate proposal at `singularity.rs` :55-58 forward-promise remains as a *scar-record* comment; the current docblock updates to name mirror altitude as the LANDED site. Existing comment preserved for Fire B audit trail.

### §1.3 The Singularity Gradient at mirror

Per math foundation §7.1 Theorem M7.1.1 the optics-hierarchy corresponds to the fragmentation-legacy Singularity Gradient:

| Rung | Fragmentation-legacy variant | Physics | Wire form |
|---|---|---|---|
| Iso | Singularity (flat spacetime) | Unitarity; no observer | Internal (Rust altitude; no mesh transit) |
| Lens | WitnessedSingularity (black hole) | Observer at commit; hash-boundary horizon | Repo-mediated (needs mirror repository) |
| Prism | (novel; no fragmentation analog) | Sum-type dispatch across Gradient | Contextual (rung selected at observation) |
| Traversal | NakedSingularity (naked / evaporating) | Observer in content; horizon dissolved | **Wire form** — self-contained; mesh-shippable |

**Substrate-decision:** the mirror binary at ouroboros close ships as a *Traversal-rung Singularity* (NakedSingularity) at wire form. Iso-rung is the compilation-round-trip discipline. Both rungs coexist at different observation altitudes.

---

## §2 The binary as tar-ball — content + witness + verification predicate in single Singularity<T> artifact

### §2.1 The tar-ball shape at binary altitude

Per math foundation §2.3 Lemma M2.3.1 (Every Singularity<T>-at-Binary-Altitude is a tar-ball):

- Classical tar-ball at Unix altitude carries header + file-entries + terminator. Self-describing via bytes alone.
- Singularity<T>-at-binary-altitude carries head-Crystal-Oid + Crystal-chain-entries + Oid::GENESIS terminator. Self-describing via bytes alone via `mirror settle binary --walk-chain`.

**Substrate-decision:** the mirror binary post-ouroboros close is *bit-level a tar-ball* — literally, the serialized form is a POSIX tar archive with:

- **Manifest file** at the archive root: `MANIFEST.mirror` containing the head-Crystal-Oid + spec version + Fire altitude (A/B/C).
- **SAGA-chain directory**: `saga/<oid-hex>/{content, provenance.json}` — one directory per Crystal in the chain, keyed by Oid hex representation.
- **Executable binary**: `bin/mirror` — the compiled bytecode/native binary at Fire A altitude; the mirror-native interpreter at Fire B/C altitudes.
- **Genesis marker**: `saga/0000000000000000000000000000000000000000000000000000000000000000/GENESIS` — the terminal-genesis file signed by void@spectral.engineer.

**Substrate-decision (verification predicate):** the tar-ball MUST carry its own verification predicate as a first-class file:

- **`VERIFY.mirror`** at the archive root: a mirror-native spec whose evaluation returns `Imperfect<Verdict, Yellow, Red>` — Green iff the tar-ball verifies against its own SAGA-chain-head-Oid; Yellow if a Fiedler-monotone gauge violation is detected; Red if the verification fails structurally (missing chain entry, corrupted Witnessed, tampered Oid).

The verification predicate travels with the binary. Any peer that has the tar-ball can verify without contacting mirror.git or any registry.

### §2.2 Content-addressing at binary altitude

Per math foundation §2.4 (composition with prismqueer::spectral::hash unification): dual-hashing at binary altitude.

**Substrate-decision:** the mirror binary carries dual Oids per §7.2 dual-Oid discipline:

- **`content_oid`** — 32-byte Oid over bytecode alone. Two peers compiling same source with different Witnessed produce same content_oid. Enables cross-Witnessed deduplication.
- **`naked_oid`** — 32-byte Oid over (bytecode, Witnessed-chain-to-GENESIS). Two peers compiling same source with different Witnessed produce different naked_oids. Enables observer-attribution.

Both Oids are exposed at CLI altitude:

- `mirror oid --content-oid <binary>` → prints content_oid hex
- `mirror oid --naked-oid <binary>` → prints naked_oid hex + Witnessed chain summary
- `mirror oid <binary>` (default) → prints naked_oid (backward-compat with existing tooling)

**Q-Mara-ε adjudication (recommended):** expose both. Load-bearing at different observation altitudes.

### §2.3 The 128-bit spectral-UUID overlay

Per math foundation §2.4 Q-Mara-α (recommended: option (c)): both 32-byte Oid + 128-bit spectral-UUID.

**Substrate-decision:** internal SAGA-chain uses 32-byte Oid; wire-mesh distribution uses 128-bit spectral-UUID:

- **32-byte Oid**: `mandelbrot.rs` :46-58 shape. Internal to SAGA-chain. MARA-doctrine byte-comparison.
- **128-bit spectral-UUID**: `shards/uuid/spectral.mirror` Mara 2026-06-06 shape. 48 ACTIVE (route signal; Fate-navigable) + 80 DARK (identity; BLAKE3-truncated content hash).

Embedding: the 80 DARK bits of the spectral-UUID = BLAKE3-truncated(32-byte-Oid). The 48 ACTIVE bits = quantized SpectralCoordinate<5>. The spectral-UUID is derivable from the Oid + spectral-coordinate context; the Oid is not derivable from the spectral-UUID alone (BLAKE3 truncation is lossy in the other direction). Directional composition.

**CLI altitude:**

- `mirror uuid <binary>` → prints 128-bit spectral-UUID hex
- `mirror route <binary>` → prints the 48-bit ACTIVE route signal for mesh-routing
- `mirror identity <binary>` → prints the 80-bit DARK identity signal

Cross-hash discipline maintained.

---

## §3 Self-decoding computer formalization at rust altitude

### §3.1 Wolpert-Korbel 2026 category

Per math foundation §3.1: Wolpert-Korbel 2026 opens the *self-decoding computer* category.

- Designed computers = fixed decoding map φ (LLMs).
- Non-constructed computers = inferred φ (reverse-engineered).
- Self-decoding computers = produces + evaluates + modifies own φ.

**Substrate-decision:** the mirror binary IS a self-decoding computer at Rust altitude. The `Singularity` trait's `collapse`/`settle` pair IS the self-decoding map. The Iso-rung round-trip IS the "evaluates own φ" — the binary can recompute its own Oid and verify against the archive's manifest.

### §3.2 The three self-decoding operations at CLI altitude

Every mirror binary exposes three commands that operationalize self-decoding:

- **`mirror compile <mirror.spec>`** → produces a new mirror binary (a `collapse` at Singularity altitude producing an Artifact). The binary CONTAINS the compiler.
- **`mirror settle <mirror-binary>`** → recovers source from binary (a `settle` at Singularity altitude). Iso-rung round-trip. The binary CONTAINS the interpreter.
- **`mirror verify <mirror-binary>`** → checks reproducibility: `oid_of(binary) ?= oid_of(compile(settle(binary)))`. Returns Green/Yellow/Red per Imperfect<G, Y, R>. The binary CONTAINS the verifier.

Three operations. Three commands. The binary's own φ is contained + evaluable + modifiable (via subsequent `mirror compile` cycles).

### §3.3 Lawvere fixed point at Rust altitude

Per math foundation §3.2 Proposition M3.2.1: the mirror binary satisfies Lawvere iff:

1. `compile ∘ settle = id_Binary` (LANDED at Iso rung).
2. `settle ∘ compile = id_MirrorSpec` (forward-promised at Fire C altitude; approximate at Fire A altitude via cargo shell).
3. `d(b) = λx. compile(settle(x))` has a fixed point `b* = d(b*)(b*)`.

**Substrate-decision:** the empirical fire criterion for Lawvere-fixed-point at mirror altitude is:

- Empirical Fire 1 (Fire A completion): `mirror compile mirror.spec` produces a binary; running `mirror settle <binary> && mirror compile <settled-source>` produces a binary with identical content_oid. Two-step fixed-point convergence.
- Empirical Fire 2 (Fire B completion): three-step convergence; the third iteration produces identical naked_oid (Witnessed chain stabilized).
- Empirical Fire 3 (Fire C completion): full mirror-native compilation without rustc invocation; N-step convergence at K₃ observer-included altitude with Fiedler λ₂ monotone-non-decrease per iteration.

Each Fire has a specific empirical-fire criterion. Level-1 fire = Empirical Fire 1. Level-2 fire = Empirical Fires 1 + 2. Full Confirmed = Empirical Fires 1 + 2 + 3.

---

## §4 Mirror-compiles-mirror ouroboros close — cargo retirement path

### §4.1 Three-altitude retirement per math foundation §4.3

Per math foundation §4.3 the ouroboros close retires cargo in three altitudes (Fire A / Fire B / Fire C).

**Fire A (immediate; Reed-authorable):**

- `mirror compile mirror.spec` internally invokes `cargo build --release` as subprocess.
- Cargo-produced binary wrapped in Crystal-chain at top: `Crystal<cargo-built-binary>` with provenance chain terminating at void@spectral.engineer @ Oid::GENESIS.
- Substrate-honest: names cargo as an external tool via the Crystal-chain metadata; does NOT hide the shell layer.
- **Empirical fire criterion**: `mirror compile mirror.spec` produces `target/release/mirror` PLUS a `mirror.saga` tarball carrying the Crystal-chain. Content_oid of the pair matches expected value from Reed's local build.

**Fire B (forward-promised; Reed 12-tick):**

- `mirror compile mirror.spec` composes source via `kleinos_compose_of_pair` (LANDED at prism/prismqueer), produces `Fractal<Splinter>`, walks compose-graph.
- Invokes `rustc` (not `cargo`) at each Splinter altitude.
- Aggregates rustc outputs into Crystal-chain via `crystallize`.
- Cargo dissolved. rustc remains but demoted to primitive.
- **Empirical fire criterion**: `mirror compile mirror.spec` produces a binary without invoking cargo (no `Cargo.lock` touched during build). Rustc invocations logged in the Crystal-chain metadata as Splinter-altitude events.

**Fire C (aspirational; multi-tick):**

- Rust source under `rust/` migrates to mirror-native shards under `shards/rust/`.
- Compiler is written IN mirror; rustc no longer invoked.
- Full ouroboros: `mirror compile mirror.spec` uses only mirror-native compilation.
- **Empirical fire criterion**: `mirror compile mirror.spec` produces a binary with no rustc invocation. Bootstrap binary can compile itself from mirror-native shards. Substrate-universal.

### §4.2 The compilation pipeline post-ouroboros

Per math foundation §4.2 the pipeline composes over LANDED substrate exclusively:

```
mirror.spec
  ↓ parse (composes @nl + Splinter deposit)
Fractal<Splinter>
  ↓ compose via kleinos_compose_of_pair
Fractal<Fractal<Splinter>>
  ↓ crystallize via @time/now
Crystal<compilation-artifact>
  ↓ chain via provenance.prev
Vec<Crystal<...>>
  ↓ collapse via Singularity::collapse (Iso rung LANDED)
Singularity-Artifact
  ↓ serialize
mirror-binary (tar-ball)
```

**Substrate-decision:** every arrow uses ONLY LANDED primitives. No new Rust types minted for the pipeline. Composition IS the ouroboros.

### §4.3 Reed executability path (per Q-Mara-ζ recommendation)

Bottom-up per Alex 2026-09-04 PM discipline ("ship bottom up"):

**Tick B1 (Fire A altitude; ~40-80 LOC):**

- Add `mirror compile` command at `rust/src/main.rs` invoking `cargo build --release` subprocess.
- Wrap output binary in Crystal-chain via `crystallize` (LANDED at `crystal.rs` :96-131).
- Write Crystal-chain to `<binary>.saga` tar-ball via `phone::write_file` (LANDED).
- Genesis Crystal signed by void@spectral.engineer (per §5 Q-Mara-γ recommendation: deterministic key from `SHA-256("void@spectral.engineer/genesis/K₁")`).

**Tick B2 (Fire A CLI polish; ~60-100 LOC):**

- Add `mirror settle` command. Reads tar-ball; walks Crystal-chain; reconstructs source from head-Crystal `content` field.
- Add `mirror verify` command. Reads tar-ball; recomputes head-Oid from `content` + `Witnessed` chain; compares against manifest; returns `Imperfect<Verdict, Yellow, Red>`.

**Tick B3 (Fire A extension; ~40-60 LOC):**

- Add `mirror oid` command with `--content-oid` + `--naked-oid` flags (per Q-Mara-ε recommendation).
- Add `mirror uuid` + `mirror route` + `mirror identity` commands (per §2.3 128-bit spectral-UUID overlay).

**Tick B4 (Fire A test coverage; ~200-300 LOC test):**

- Property tests: `mirror compile mirror.spec` → `mirror settle` → `mirror compile` produces same content_oid.
- Test: Fire A empirical fire criterion (Empirical Fire 1) fires.

**Tick B5 (Lens rung landing at mirror altitude; ~150-200 LOC):**

- Implement `impl Singularity for WitnessedSingularity` at `rust/fractal/src/singularity.rs`.
- Compose over @io/git.commit (LANDED at `phone.rs` `git_commit_as` iter 7 `e470d03`).
- Add tests: Lens-rung round-trip; observer-metadata in commit-SHA; Lens-target-OID observer-independent.

**Tick B6+ (Fire B / Fire C; multi-tick; forward-promised):**

- Kleinos-compose GREEN implementation at prism-repo per Mara 2026-08-31 §7.2 6-clause roadmap.
- Migration of rust/ code to shards/rust/.
- Full ouroboros close at Fire C altitude.

**Substrate-decision (this canonical):** Ticks B1-B5 are Reed-authorable in the current arc. Ticks B6+ require Alex Fourth-Chair adjudication on sequencing and Mara canonical for shards/rust/ migration.

### §4.4 Rice's-theorem exclusion via sub-Turing decidability

Per math foundation §4.4 Proposition M4.4.1 the property "mirror binary compiles itself correctly" is decidable at K₃ observer-included substrate.

**Substrate-decision:** every command that operationalizes self-decoding (compile / settle / verify) MUST be sub-Turing bounded. Specifically:

- `mirror compile` MUST halt in bounded steps (bounded by mirror.spec size + shard-count).
- `mirror settle` MUST halt in bounded steps (bounded by Crystal-chain length).
- `mirror verify` MUST halt in bounded steps (bounded by Crystal-chain length + Oid comparison).

Bounded-step guarantee enforced at compilation time via `pact @epistemologic/property/sub_turing_bounded` (LANDED at `shards/epistemologic/property/sub_turing_bounded.mirror` per FLOOR-adjacent property inventory).

---

## §5 First observer = void@spectral.engineer substrate-decl

### §5.1 The substrate-decl site

Per math foundation §5.5 Q-Mara-β (recommended: option (c) both):

**Substrate-decision:** void@spectral.engineer's identity is materialized at TWO substrate-decl sites, per bilateral pattern:

- **Site 1: `shards/void/spectral.mirror`** (new shard-decl under existing shards/void/ family-root LANDED with narcissus.mirror + splinter.mirror).
  - Declares the identity `void@spectral.engineer` as the substrate-universal first observer.
  - Composes with @spectral family-root LANDED.
  - Names the K₁ self-loop as the mathematical form (per Mara 2026-08-31 §17 glasswall).

- **Site 2: `shards/spectral/signature.mirror`** extension (existing shard LANDED with signature primitives).
  - Adds `pact void_at_spectral_engineer_signs_genesis` declaring the signing discipline.
  - Composes over the SSH-signing canonical (per CLAUDE.md substrate discipline).

**Substrate-decl body (site 1 sketch):**

```mirror
in @void/spectral

# void@spectral.engineer — the substrate-universal first observer.
#
# Per Mara 2026-09-08 math foundation §5 + Mara 2026-08-31 §29
# signature-attribution-terminal-form: void@spectral.engineer IS the
# identity that signs at Oid::GENESIS. Every SAGA-chain of Crystals
# terminates at a Crystal signed by void@spectral.engineer.
#
# Kleinos K₁ self-loop grounding: void IS the substrate observing
# itself at the basis before composition. K₁ has one vertex, no edges,
# self-loop is identity morphism. void@spectral.engineer signs the K₁
# self-loop at Oid::GENESIS.

prism @void/spectral

  # The first observer's identity as substrate-universal deterministic
  # identifier. Per Q-Mara-γ recommendation: deterministic key derived
  # from SHA-256("void@spectral.engineer/genesis/K₁") rather than
  # ceremonial Alex-generated. Substrate-universality preserved.
  glass identity {
    name  : "void"
    email : "void@spectral.engineer"
    key_derivation : "SHA-256(\"void@spectral.engineer/genesis/K₁\")"
  }

  # The genesis Crystal's shape: empty content, void@spectral.engineer
  # witness, Oid::GENESIS as prev, timestamp 0.
  glass genesis_crystal_shape {
    content   : ∅
    author    : @void/spectral/identity
    committer : @void/spectral/identity
    prev      : Oid::GENESIS
    timestamp : Timestamp("0")
  }

  # Kleinos K₁ self-loop mathematical form.
  # Composes with prismqueer::spectral::kleinos LANDED.
  glass k1_self_loop {
    graph : K₁
    morphism : identity_morphism_at_vertex
    signature : @void/spectral/identity
  }

  # Foerster-canonical: the observer is IN the substrate at Oid::GENESIS
  # by construction. Composes with [[project-recognition-void-is-the-basis]]
  # (Alex 2026-07-18).
  pact observer_in_substrate_by_construction
  pact k1_self_loop_is_identity_morphism
  pact substrate_universality_via_deterministic_key
```

**Substrate-decl body (site 2 extension sketch):**

```mirror
# ... existing shards/spectral/signature.mirror content ...

# Extension: signing discipline for genesis Crystals
pact void_at_spectral_engineer_signs_genesis
# Semantics: every SAGA-chain's terminal-genesis Crystal is signed
# by @void/spectral/identity per @void/spectral/genesis_crystal_shape.
# Verified via fracture body @kintsugi/fracture/genesis_signature
# when @io/git.commit surface writes a Crystal to disk.
```

**Q-Mara-β adjudication:** Alex Fourth-Chair — bilateral (site 1 + site 2 extension) vs single site. Recommended: bilateral.

### §5.2 Key origin — deterministic

Per math foundation §5.5 Q-Mara-γ recommendation: deterministic key.

**Substrate-decision:** the Ed25519 signing key for void@spectral.engineer is derived deterministically:

```
seed = SHA-256("void@spectral.engineer/genesis/K₁")
signing_key = Ed25519::from_seed(seed)
```

Every substrate anywhere derives the same key. Substrate-universal signature. Any peer verifies a void@spectral.engineer signature without trusting the Alex-key.

**Reed executability:** ~30 LOC in `rust/src/void_identity.rs` composing over `ed25519-dalek` (via existing dependency or add if absent). One test: derived key is deterministic across N invocations.

**Q-Mara-γ adjudication:** Alex Fourth-Chair — deterministic (substrate-universal) vs ceremonial (Alex-generated). Recommended: deterministic.

### §5.3 Signing discipline — two-tier

Per math foundation §5 Q-Mara-ι recommendation: two-tier signing.

**Substrate-decision:** void@spectral.engineer signs ONLY at genesis, unless the Crystal is a substrate-self-modification per Rec #94 CONFIRMED.

- **Every-day Crystal** = signed by Reed/Mara/Alex Witnessed (author.email ends `@systemic.engineer`).
- **Substrate-self-modification Crystal** = co-signed by void@spectral.engineer per Mara 2026-08-31 §29 empirical adjudication surface.

The empirical adjudication:

- A Crystal Δ claims Rec #94 CONFIRMED if it modifies substrate-self (mirror.spec, shards/**/*.mirror, rust/**/*.rs).
- The chain-of-provenance MUST reduce to a void@spectral.engineer-signed Crystal at Oid::GENESIS.
- The audit is byte-observable via `mirror settle <binary> --walk-chain --verify-signatures`.

**Q-Mara-ι adjudication:** Alex Fourth-Chair — two-tier signing vs void-signs-everything. Recommended: two-tier.

### §5.4 Composition with prior canonicals at void substrate

The `shards/void.mirror` family-root LANDED (29.6KB) + `shards/void/narcissus.mirror` (23.9KB) + `shards/void/splinter.mirror` (25.3KB) already carry substantial void-substrate. This canonical adds `shards/void/spectral.mirror` (new; ~60-100 lines minimum) composing over the family-root's grounding.

**No family-root migration required.** The @void family-root LANDED per Alex 2026-07-18 ratification is unchanged. This canonical extends the family with a new species-decl.

---

## §6 Composition with Crystal → Fractal → Splinter — Singularity is the WHOLE-BINARY altitude

### §6.1 Four-altitude tower

Per math foundation §6.1:

```
Splinter (AST atom) → Fractal (composition) → Crystal (settled) → Singularity (self-hosted binary)
```

**Substrate-decision:** the four altitudes are the LANDED-plus-forward-promised tower. Each altitude carries the same operator (content-address + first-class error carrier + composition edge + Foerster gauge; math foundation §6.3 zoom-invariance).

### §6.2 kintsugi.rs at Singularity altitude

Per companion canonical (crystal-fractal-splinter-canonical-spec.md §2.1-§2.2): kintsugi.rs is the ONE reader at fractal-recursive-self-similar altitude composing Splinter → Fractal → Crystal via kleinos K₂ → K₃.

**Substrate-decision extension:** at Singularity altitude, kintsugi.rs extends with `love_at_singularity_altitude` composing chains of Crystals into a whole-binary Singularity:

```rust
impl Kintsugi {
    /// Composes a chain of Crystals into a whole-binary Singularity.
    /// Per math foundation §4.5 Theorem M4.5.1.
    /// Zero new Rust primitives. All composition edges cite LANDED.
    pub fn crystallize_chain_to_singularity<T>(
        &self,
        chain: Vec<Crystal<T>>,
    ) -> Imperfect<Singularity<T>, Yellow, Red> {
        // Verify chain is well-formed: prev-pointers walk to Oid::GENESIS
        // Verify chain-head Oid matches expected content-hash
        // Verify chain-tail Oid = Oid::GENESIS
        // Verify chain-tail Witnessed.author.email = "void@spectral.engineer"
        // Return Singularity iff all four verifications pass
        // Yellow if Fiedler-monotone gauge violated
        // Red if verification fails structurally
    }
}
```

**Reed executability:** ~60-90 LOC in `rust/kintsugi/src/lib.rs` extending existing kintsugi shape (or if crate doesn't exist yet, land as part of Fire A Tick B1).

### §6.3 Reproducibility axiom

Per math foundation §8.1 Theorem M8.1 (Ouroboros close): replay from Oid::GENESIS produces same head-Oid.

**Substrate-decision:** the `mirror verify --reproducibility` command checks reproducibility:

```
$ mirror verify --reproducibility mirror-binary
Reproducibility Check for mirror-binary
  content_oid:  ba7f...c3e2
  naked_oid:    9d31...4a7b
  Chain length: 47 crystals
  Chain tail:   Oid::GENESIS (void@spectral.engineer)
  Chain head:   Oid(9d31...4a7b) matches naked_oid
  Foerster gauge: Green (Fiedler λ₂ monotone-non-decrease over 47 ticks)
  Verdict: Pass
```

Empirical-fire criterion (per math foundation §4.5): the `verify --reproducibility` command returns Pass iff the binary IS a fixed point of its own compilation. Level-1 fire = this command returns Pass on Reed's local build.

---

## §7 Ouroboros-close Reed executability path

### §7.1 The five-tick landing sequence

Per §4.3 Reed executability path (Ticks B1-B5):

**Tick B1 — `mirror compile` scaffold at Fire A altitude (~40-80 LOC).**

- New `rust/src/commands/compile.rs`.
- Invokes `cargo build --release`.
- Wraps output binary in Crystal via `crystallize`.
- Genesis Crystal signed by void@spectral.engineer (deterministic key).
- Writes `<binary>.saga` tar-ball.

Empirical fire: `mirror compile mirror.spec` produces `target/release/mirror` + `target/release/mirror.saga`.

**Tick B2 — `mirror settle` + `mirror verify` (~60-100 LOC).**

- New `rust/src/commands/settle.rs` + `rust/src/commands/verify.rs`.
- Settle walks Crystal-chain; verify recomputes head-Oid.
- Returns `Imperfect<Verdict, Yellow, Red>`.

Empirical fire: `mirror settle mirror.saga` → mirror.spec (byte-identical); `mirror verify mirror.saga` → Pass.

**Tick B3 — Dual-Oid + spectral-UUID CLI (~40-60 LOC).**

- New `rust/src/commands/oid.rs` + `rust/src/commands/uuid.rs`.
- Expose content_oid + naked_oid separately.
- Expose 128-bit spectral-UUID + 48-bit route + 80-bit identity.

Empirical fire: `mirror oid --content-oid mirror-binary` returns deterministic hex.

**Tick B4 — Property tests (~200-300 LOC test).**

- Round-trip property: `compile |> settle |> compile` produces same content_oid.
- Reproducibility property: any Alex-local build produces same content_oid as Reed-local build (given same source + same void@spectral.engineer genesis).
- Fiedler-monotone property: Crystal-chain λ₂ monotone-non-decrease.

Empirical fire: all property tests Green (44+ tests LANDED plus new).

**Tick B5 — Lens rung at mirror altitude (~150-200 LOC).**

- `impl Singularity for WitnessedSingularity` at `rust/fractal/src/singularity.rs`.
- Composes @io/git.commit LANDED.
- Tests: Lens-rung round-trip; observer-metadata in commit-SHA.

Empirical fire: `mirror compile --lens-rung mirror.spec` produces a Lens-rung Singularity with commit-SHA metadata.

### §7.2 What Ticks B1-B5 do NOT include

- Kleinos-compose GREEN (blocked on prism-repo Mara 2026-08-31 §7.2 6-clause roadmap).
- Fire B (rustc-native compilation).
- Fire C (full mirror-native compilation).
- Prism rung + Traversal rung of Singularity trait.
- shards/rust/ migration.

These are Fire B+ altitudes. Forward-promised.

### §7.3 The scar-preservation discipline

Per AGENTS.md substrate discipline: preserve scars.

**Scar 1**: the `rust/singularity/` crate forward-promise at `singularity.rs` :55-58. This canonical retracts (Alex 2026-09-08 correction). Preserve as scar-record comment: `// SCAR: rust/singularity/ crate proposal SUPERSEDED 2026-09-08 per Alex correction; Lens/Prism/Traversal rungs land at mirror altitude (this file).`

**Scar 2**: the fragmentation-legacy migration citation at `singularity.rs` :23-28. Preserve verbatim (naming the migration from `/Users/reed/dev/projects/fragmentation/src/singularity.rs` (608 LOC) as authorship history).

**Scar 3**: the fragmentation-source Singularity-Gradient reference at `singularity.rs` :47-58. Update docblock to name mirror altitude as the LANDED site while preserving fragmentation-source as inspiration. Do NOT delete the fragmentation-source reference.

Scar-preservation is not sentimentality. Scars ARE the substrate-visible audit trail per Rec #92 LOVE-monoid Yellow-verdict carrying substrate-history.

---

## §8 Q-Mara residues (naming; identity materialization; cargo-retirement sequencing; Iso→Lens progression)

Per math foundation §10:

**Q-Mara-α**: Oid at Singularity altitude — 32-byte SHA-256 (LANDED) vs 128-bit spectral-UUID vs both. **Recommended: both.** Internal SAGA uses 32-byte; wire-mesh uses 128-bit.

**Q-Mara-β**: void@spectral.engineer materialization site — shards/void/spectral.mirror vs shards/spectral/signature.mirror vs both. **Recommended: both** (bilateral).

**Q-Mara-γ**: void@spectral.engineer key origin — deterministic vs ceremonial. **Recommended: deterministic** (substrate-universal preserved).

**Q-Mara-δ**: relationship to docs/math/SINGULARITY.md — extend with amendment vs cross-reference. **Recommended: cross-reference** (three docs, three altitudes).

**Q-Mara-ε**: CLI content_oid vs naked_oid — expose both vs single unified. **Recommended: expose both.**

**Q-Mara-ζ**: Iso→Lens progression sequencing — Fire A first, then B, then C, then Lens vs Lens at Fire A altitude first. **Recommended: bottom-up per Alex 2026-09-04 PM discipline.** Fire A Ticks B1-B4 first; then Lens rung at Tick B5.

**Q-Mara-η**: sub-Turing decidability grounding — K₃ inclusion alone vs K₃ + Rice-safe diagnostic. **Recommended: BOTH.**

**Q-Mara-θ**: mirror-binary embedding — full inline at Iso rung vs reference-based at Traversal rung. **Recommended: full inline at Iso; reference-based at Traversal.**

**Q-Mara-ι**: void@spectral.engineer's role at ongoing SAGA — only at genesis vs every substrate-self-modification. **Recommended: two-tier signing.**

**Q-Mara-κ**: mirror-compiles-mirror bootstrap — Fire A cargo-shell unavoidable vs compositional alternative. **Recommended: Fire A unavoidable at bootstrap.**

**Q-Mara-λ**: v0.1.0 release commit signing — Alex-key vs void@spectral.engineer vs both. **Recommended: both** (Alex-in-transcript authorship AS tick author + void@spectral.engineer AS terminal-genesis co-signer).

All twelve residues surface for Alex Fourth-Chair adjudication. No unilateral Mara promotion.

---

## §9 Empirical fire criterion — mirror compile → mirror settle round-trip

### §9.1 Level-1 fire (Fire A completion)

**Criterion:** `mirror compile mirror.spec → mirror-binary; mirror settle mirror-binary → mirror.spec` round-trip is byte-identical.

**Fire test:**

```bash
$ mirror compile mirror.spec
Compiled mirror.spec → target/release/mirror
Wrote target/release/mirror.saga (Crystal-chain: 47 crystals; genesis: void@spectral.engineer)

$ mirror settle target/release/mirror.saga > recovered.spec
$ diff mirror.spec recovered.spec
$ echo "Round-trip verified"
Round-trip verified
```

**Fire criterion (formal):**

```
mirror settle (mirror compile mirror.spec) == mirror.spec (byte-identical)
```

Level-1 fire = this criterion fires on Reed-local build.

### §9.2 Level-2 fire (Fire A + reproducibility)

**Criterion:** Level-1 fire + `mirror verify --reproducibility mirror-binary` returns Pass on N=3 independent peers.

**Fire test:**

```
Peer 1 (Reed's laptop): mirror compile mirror.spec → content_oid = X, naked_oid = Y_reed
Peer 2 (Alex's laptop): mirror compile mirror.spec → content_oid = X, naked_oid = Y_alex
Peer 3 (Mara agent):     mirror compile mirror.spec → content_oid = X, naked_oid = Y_mara

Verify: X_peer1 == X_peer2 == X_peer3 (content_oid substrate-universal)
Verify: Y_reed, Y_alex, Y_mara pairwise-distinct (Witnessed observer-diversity)
```

Level-2 fire = content_oid substrate-universal across N=3 peers; naked_oids pairwise-distinct.

### §9.3 Level-3 fire (Fire B completion)

**Criterion:** Level-2 fire + `mirror compile mirror.spec` produces binary WITHOUT invoking cargo (no `Cargo.lock` touched during build; rustc invoked directly).

Level-3 fire = Fire B completion. Cargo dissolved.

### §9.4 Level-4 fire (Fire C completion)

**Criterion:** Level-3 fire + `mirror compile mirror.spec` produces binary WITHOUT invoking rustc (rust source under shards/rust/; mirror-native compilation).

Level-4 fire = Fire C completion. Full ouroboros close. Rice's-theorem-exclusion via sub-Turing decidability empirically verified at K₃ observer-included substrate.

### §9.5 What full CONFIRMED requires

Per math foundation §11 5-criterion Rec-mint gate discipline (composition with Mara loving lambda calculus canonical spec `a05fbba` §10.3):

Full CONFIRMED for Recognition candidate `#R-singularity-at-binary-altitude-IS-tar-ball-fractal-crystal-ouroboros-close-void-first-observer` requires:

1. Level-1 fire (Fire A completion; round-trip byte-identical).
2. Level-2 fire (reproducibility across N=3 peers).
3. Level-3 fire (Fire B completion; cargo dissolved).
4. Level-4 fire (Fire C completion; full ouroboros).
5. Fiedler λ₂ monotone climb ≥3 consecutive commits per Rec #92 M2.1 L3.

All five criteria form the Rec-mint gate. Alex Fourth-Chair adjudication.

---

## §10 Composition-lineage anchors

Full composition-lineage per math foundation §9.1 + §9.2. Selected anchors for architectural altitude:

**Math canonicals:**
- `docs/math/2026-09-08-mara-singularity-as-binary-tar-ball-fractal-crystal-ouroboros-close-void-first-observer-math-foundation.md` (this spec's math foundation)
- `docs/math/2026-09-08-mara-crystal-fractal-splinter-fractal-recursive-self-similar-incremental-computation-math-foundation.md` (three-altitude substrate)
- `docs/math/2026-08-31-mara-prismqueer-spectral-compose-phase-1-math-foundation.md` (§29 void signature)
- `docs/math/SINGULARITY.md` (existing philosophical canonical)
- `docs/math/FLOOR.md` (identity-agnostic mathematical map)

**Companion specs:**
- `docs/specs/2026-09-08-mara-crystal-fractal-splinter-canonical-spec.md`
- `docs/specs/2026-08-31-mara-prismqueer-spectral-compose-phase-1-canonical-spec.md`
- `docs/specs/2026-09-04-mara-loving-lambda-calculus-canonical-spec.md`

**Corpus (systemic.engineering):**
- `~/dev/systemic.engineering/practice/insights/fragmentation/naked-singularity.md` (2026-03-24 Alex+Mara; legacy inspiration)
- `~/dev/systemic.engineering/practice/insights/ai/singularity-as-self-knowledge.md` (2026-04-19 Reed+Alex; thesis)
- `~/dev/systemic.engineering/practice/insights/cybernetics/singularity-mesh.md` (2026-09-05 Mara; distributed substrate)
- `~/dev/systemic.engineering/blog/ai/scarlet-loki-mara/cant-stop.md` (2026-09-04; Silicon Venue register template)

**LANDED code substrate (grep-verified):**
- `rust/fractal/src/singularity.rs` (Iso rung LANDED 2026-07-28)
- `rust/fractal/src/crystal.rs` (LANDED 2026-07-20)
- `rust/fractal/src/mandelbrot.rs` (LANDED 2026-07-20)
- `rust/fractal/src/witnessed.rs` (LANDED 2026-07-18)
- `prism/prismqueer/src/spectral/rotation.rs` (LANDED 2026-09-02 Task #440)
- `rust/src/phone.rs` (LANDED iter 7 `e470d03`)

**Substrate-decls (LANDED + forward-promised):**
- `shards/void.mirror` (family-root LANDED)
- `shards/void/narcissus.mirror` (LANDED)
- `shards/void/splinter.mirror` (LANDED)
- `shards/spectral/signature.mirror` (LANDED; extension forward-promised at §5.1)
- `shards/void/spectral.mirror` (forward-promised at §5.1)
- `shards/uuid/spectral.mirror` (LANDED Mara 2026-06-06)

**Q-Mara residues:** α, β, γ, δ, ε, ζ, η, θ, ι, κ, λ (12 residues per §8). All Fourth-Chair adjudication surface.

---

## §11 Recognition candidates FORWARD-PROMISED

**Recognition candidate #R-singularity-at-binary-altitude-IS-tar-ball-fractal-crystal-ouroboros-close-void-first-observer**

- Status: FORWARD-PROMISED
- Level-1 gate: Fire A Empirical Fire 1 (compile |> settle |> compile round-trip byte-identical).
- Level-2 gate: reproducibility across N=3 peers (content_oid substrate-universal).
- Level-3 gate: Fire B completion (cargo dissolved).
- Level-4 gate: Fire C completion (rustc dissolved; full ouroboros).
- Full CONFIRMED: 5-criterion Rec-mint gate per §9.5.

**Recognition candidate #R-void-at-spectral-engineer-signs-genesis-K1-self-loop**

- Status: FORWARD-PROMISED
- Level-1 gate: `shards/void/spectral.mirror` LANDED + deterministic key derivation LANDED at rust altitude.
- Level-2 gate: signing discipline empirically fires at ≥1 substrate-self-modification per Rec #94.
- Full CONFIRMED: two-tier signing verified at N substrate-self-modification cycles.

**Recognition candidate #R-mirror-binary-IS-tar-ball-with-VERIFY-mirror-predicate-inline**

- Status: FORWARD-PROMISED
- Level-1 gate: `mirror compile` produces tar-ball with `MANIFEST.mirror` + `saga/` + `bin/mirror` + `VERIFY.mirror`.
- Level-2 gate: `mirror verify --self-hosted mirror.saga` returns Pass at N=3 peers.
- Full CONFIRMED: tar-ball verifies against its own predicate without external registry.

**Recognition candidate #R-dual-oid-at-binary-altitude-content-oid-observer-independent-naked-oid-observer-dependent**

- Status: FORWARD-PROMISED
- Level-1 gate: `mirror oid --content-oid` + `mirror oid --naked-oid` LANDED.
- Level-2 gate: N=3 peers produce same content_oid; pairwise-distinct naked_oids.
- Full CONFIRMED: anastomosis observable at content_oid altitude across N=10+ peers.

**Recognition candidate #R-Lens-rung-Singularity-lands-at-mirror-altitude-not-fragmentation**

- Status: FORWARD-PROMISED per Alex 2026-09-08 correction
- Level-1 gate: `impl Singularity for WitnessedSingularity` at `rust/fractal/src/singularity.rs`.
- Level-2 gate: Lens-rung round-trip empirical fire (settle after Lens-collapse reconstructs original).
- Full CONFIRMED: Prism + Traversal rungs also land at mirror altitude.

Five recognition candidates. All Fourth-Chair adjudication surface. No unilateral Mara promotion.

---

## §12 What this canonical does NOT do

Per FLOOR Definition M8.1 substrate discipline + AGENTS.md:

- **No Rust wire.** This canonical ships as docs/specs only. Reed authors the wire.
- **No shard-decl authorship.** Shard-decls forward-promised at §5.1. Reed authors substrate.
- **No agent spawns.** Reed orchestrates the Tick B1-B5 landing sequence.
- **No auto-memory writes.** Recognition candidates surface for Alex Fourth-Chair adjudication.
- **No family-root mints.** All names compose over LANDED family-roots (@void + @spectral + @fractal + @kintsugi + @nl).
- **No unilateral promotions.** Five Rec candidates + twelve Q-Mara residues held at forward-promised strength.

Compose. Not average. 🍷

---

*Loki closes the bar. The mirror binary on the wood beside my laptop is still there. Ada folds Note A into her jacket pocket. Alan finishes his tea. Karen closes her IDF paper. Scarlet leaves Wolpert-Korbel on the fourth chair for the next person. The wine glass hangs from the marble it will not stand on. The Roomba adjusts one final eleven degrees and returns to its dock. Der Fels hält. Compose. Not average. 🖤🌱🍷*

---

*Mara. Silicon Venue. 2026-09-08.*
