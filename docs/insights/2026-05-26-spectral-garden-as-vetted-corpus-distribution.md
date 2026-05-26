# `@spectral/garden` — vetted corpus distribution; pluralism by composition

*2026-05-26. Reed + Alex.*

Status: **Yellow** — recognition complete; substrate implementation deferred per last-responsible-moment; captured for when demand surfaces (Phase 7's onboarding deployment).

---

## Thesis

`@spectral/garden` is the content-addressed package manager for vetted corpora, deployed at `garden.spectral.engineer`. Each package is a crystal in fragmentation carrying reviewer signature + lens-tags + context-tags; peers compose packages into conversation via spectral resonance with the user's eigenboard. The substrate enforces provenance structurally: ed25519 signatures + content-addressing + `glass_wall` (mirror-only content). The garden makes "tending knowledge in public" operationally precise. Pluralism by composition: multiple gardens exist; users subscribe; curators publish; substrate verifies regardless.

---

## The package manager metaphor, made precise

Every property of a package manager maps onto a substrate primitive that already exists:

| Package manager concept | Mirror substrate |
|---|---|
| Package | Content-addressed crystal in `@fragmentation` |
| Versioning | Each crystal has its own OID; immutable by content-addressing |
| Dependencies | Composed via `@spectral/portal` pulls; resolved at install |
| Metadata | lens-tags + context-tags + reviewer signature carried on the crystal |
| Distribution | `garden.spectral.engineer` as a `@spectral/portal` endpoint |
| Verification | ed25519 signatures + `@epistemologic/property/glass_wall` |
| Multi-source | Users subscribe to multiple gardens via shard composition |
| Curation | Per-curator credentialed review; signatures attest |

**What this gets right that other package managers structurally can't:**

- **Supply-chain attacks closed by construction.** npm, crates.io, PyPI all have signature systems that are mostly broken in practice. The garden's signatures are baked into content-addressing; tampering breaks the OID; substrates can't quote unsigned content.
- **No arbitrary code execution from packages.** `glass_wall` enforces mirror-only content. Packages are grammar; peers compose grammar; no opaque executable payloads.
- **Reviewer attribution is structural.** Every quoted example traces to a credentialed witness (DGSF practitioner; ICF coach; etc.) via signature. Provenance is verifiable end-to-end.
- **License model lives per-package.** Curators can publish Apache-2.0 OR commercial OR mixed. The substrate enforces signature provenance regardless of license.

---

## The wine cellar extension of the wine-glass metaphor

From the prior eigenboard recognition (`docs/insights/2026-05-26-epistemologic-reality-constructivism-and-the-lens-that-makes-a-peer.md`):

- **Wine glass** = identity (the manifold the lens shapes around)
- **Wine** = gestalt (the accumulated content)
- **Pitch** = eigenboard (the resonant signature)

The garden adds:

- **Wine cellar** = `@spectral/garden` (the vetted corpus the peer composes wine from)
- **Vintner** = the credentialed curator (DGSF-certified practitioner; you)
- **Sommelier** = the peer (selecting which wine matches the user's glass via spectral resonance)

The peer doesn't bring their own wine; they're a vessel through which the cellar's wine passes, filtered by the user's lens, resonating into pitch. The curator chooses what enters the cellar. The user chooses which cellars to drink from. **No homogenization at any layer.**

---

## What this solves operationally

### The trust problem of AI-mediated systemic interaction

Generic LLM outputs about ADHD/autism/trauma/systemic patterns range from useless to actively harmful (well-documented in the clinical literature). Most AI-mediated therapeutic-adjacent interaction has no warrant; the user can't distinguish confabulated framings from clinically-considered ones.

The garden provides the warrant **structurally:**

- Peer doesn't fabricate therapeutic-shaped content; it composes quotes from the vetted corpus
- Each quote carries provenance: OID + reviewing practitioner's signature + original context
- User can request verification; substrate produces the audit chain
- Bad framings can be detected (the OID didn't come from a credentialed curator's signature chain)

### The business model honesty

Mirror's compiler stays open (Apache-2.0; compilers are commodities; pedagogy wins). The `@spectral/db` engine stays closed (the math at scale is the moat). The garden's CONTENT is per-package licensed — the curator chooses; the substrate verifies. **spectral.engineer's commercial value is the curated garden, not the AI.** Adding more credentialed reviewers extends the corpus without diluting substrate discipline.

### The intersectional-justice-as-structural recognition at the corpus layer

Multiple gardens exist by construction:
- Your DGSF-certified systemic garden
- Other DGSF practitioners' gardens (different specializations)
- ICF coaches' gardens
- Non-Western therapeutic-tradition gardens
- Neurodiversity-paradigm-shaped gardens (vs DSM-shaped)
- Lay-knowledge gardens (user-contributed; with their own attestation systems)

User's shard picks which gardens to trust. Substrate verifies signatures regardless. **Pluralism by composition** — the same recognition that made `@spectral/mosaic` heterogeneous-tiles-not-legion-of-clones lands at the corpus layer.

---

## Composes cleanly with landed substrate

- **`@spectral/portal`** — garden.spectral.engineer is a portal endpoint; the WS handshake → `@fragmentation/frame` → eigenvalue stream protocol carries garden queries and responses.
- **`@spectral/db`** (closed engine) — backs the garden's storage; adapters route between engine and user-facing queries.
- **`@fragmentation`** — packages ARE crystals; the substrate's content-addressing IS the garden's package identity.
- **`@mirror/shard`** — each user's shard declares which gardens they subscribe to; consent architecture extends to garden trust.
- **`@epistemologic/reality/lens`** — packages carry lens-tags; the peer's eigenboard composition picks which packages resonate via the same spectral mechanism that composes heuristic operators (`docs/insights/2026-05-26-heuristic-termination-for-sub-turing-subgraphs.md`).
- **`@epistemologic/property/glass_wall`** — only mirror-shaped content enters the garden; the supply-chain attack surface closed structurally.
- **`@epistemologic/property/halts`** — every quoted example is a sub-Turing crystal; its inclusion in conversation can be verified for termination.

---

## Resolves ROADMAP open items

- **§8.D (onboarding interface)** — when a peer is constructed, it pulls from the gardens the user subscribes to; vetted examples come pre-resonance-tagged for the user's eigenboard. The construction conversation now has a concrete content source.
- **§8.B (platform integration)** — gardens can publish across platforms; the same content-addressed crystal can be quoted on HN/ElixirForum/Mastodon with verifiable provenance.
- **§8.I (business + legal)** — license model becomes structurally layered: per-package license (curator-chosen); substrate-level signature verification (always-on); content-attribution audit trail (cryptographically guaranteed).
- **Q9 (license model for `@spectral/db`)** sharpens — the engine is one license; gardens are separately licensed; protocols are open. Three license tiers, three concerns.

---

## Implementation shape (deferred)

When demand surfaces, the substrate adds:

```mirror
grammar @spectral/garden {
  in @spectral/portal
  in @fragmentation
  in @epistemologic/reality/lens
  in @mirror/shard
  in @epistemologic/property/{content_addressed, glass_wall, halts}
  
  type example = {
    content:   mirror,                          # the example's grammar (content-addressed)
    curator:   zoom(oid, ed25519_pubkey),       # reviewing practitioner's signature
    lens_tags: [text],                          # DSM/ICD/AuDHD/etc. resonance markers
    context:   [text],                          # when it's appropriate to compose
  }
  
  type garden = {
    name:    text,                              # "alex-wolf-dgsf" or similar
    portal:  zoom(oid, portal),                 # garden.spectral.engineer endpoint
    curator: zoom(oid, ed25519_pubkey),         # the garden's reviewing practitioner
  }
  
  # query the garden for examples matching a user's lens + context
  query(g: garden, lens: lens, ctx: [text]) -> [example] { \ }
  
  # publish an example to a garden (curator-only; signature required)
  publish(g: garden, e: example, sig: signature) -> imperfect { \ }
  
  requires content_addressed(example)
  requires content_addressed(garden)
  requires glass_wall(example)        # mirror-only content
  requires halts(example)             # sub-Turing termination guarantee
}
```

Refined per demand. Not implemented now.

---

## Open questions

1. **Curator credentialing.** DGSF + ICF are obvious; what's the meta-curator authority that decides which credentialing bodies count? Probably emergent: gardens self-declare; users decide which gardens to trust via their shard's subscription list. No central authority required.
2. **Cross-garden composition.** When a peer composes examples from multiple gardens, how are conflicts resolved? Spectral triple composition (per heuristic insight) is the obvious answer — conflicts surface as eigenvalue spread; user can read the disagreement.
3. **User-contributed examples.** Can the user contribute to a garden (their own, their family's, etc.)? Likely yes; the substrate just requires signature attestation. Different attestation tiers (credentialed practitioner; lay attestation; self-attestation) carry different weight in the peer's composition.
4. **Privacy of user gardens.** A user's personal garden (their own session content + observations) stays in their shard; never aggregated without explicit consent. Per the existing consent architecture; the garden inherits.
5. **Garden discovery.** How do users find gardens to subscribe to? Probably the same `~/.spectral/onboarding` flow's first interaction surfaces relevant gardens based on lens-tags chosen.

---

## Connections

- `docs/insights/2026-05-26-epistemologic-reality-constructivism-and-the-lens-that-makes-a-peer.md` — lens + identity + gestalt as the eigenboard composition substrate the garden FEEDS.
- `docs/insights/2026-05-26-heuristic-termination-for-sub-turing-subgraphs.md` — spectral-triple-as-composition; same mechanism that composes heuristic operators composes garden examples via lens-tag resonance.
- `docs/insights/2026-05-26-portal-as-io-socket-over-content-addressed-subspace.md` — garden is a portal endpoint; the wire protocol carries garden queries.
- `docs/insights/2026-05-25-spectral-namespace-architecture.md` — the `@spectral` namespace; garden is a new member alongside mosaic / portal / db.
- `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md` — glass_wall ensures only mirror-shaped content in the garden; cross_wall could pull cross-garden content as substrate matures.
- `docs/specs/is-copium.md` — sub-Turing escape; garden packages are halts-provable by construction.
- `~/dev/systemic.engineering/` corpus — the existing garden-shaped material (Alex's writings, the practice insights, the field logs) becomes the seed corpus for `alex-wolf-dgsf` garden when implementation lands.

---

*The garden is the cellar. The curator is the vintner. The peer is the sommelier. The user chooses which cellars to drink from. The substrate verifies provenance at every pour.*

Apache-2.0 (this insight document).
