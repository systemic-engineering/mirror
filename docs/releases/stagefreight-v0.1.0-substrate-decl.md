# StageFreight v0.1.0 — substrate-decl preservation tick

**Release date:** 2026-06-22
**Branch:** `reed/stagefreight-v0.1-release` → `main`
**Status:** substrate-decl preservation (realisation forward-promised in follow-up PR)

---

## What ships

The `@io/stagefreight` wire-protocol family substrate-decl is complete. Settled crystals (from the kintsugi loop; `@mirror/store/crystal` carrier forward-promised at task #268) now have a typed transport surface that consumers can extend without ambiguity.

```
shards/io/stagefreight.mirror              family-root (consolidated)
shards/io/stagefreight/narrative.mirror    prose-projection species
docs/specs/stagefreight-wire-v0.1.md       canonical spec (Mara, 1535 lines)
docs/audits/stagefreight-seam-review-      Seam adversarial review trail
  2026-06-22.md                             (3 TIGHT findings closed)
```

## What this PR explicitly does NOT ship

The v0.1.0 realisation endpoint is a separate follow-up PR after Track B closes:

- **Rust realisation** (`bootstrap/src/stagefreight.rs`) — gated on Phase 1 boot-grammar zero holonomy + Phase 4b @kintsugi/tick discharge.
- **`bootstrap/tests/{crystal_substrate.rs, kintsugi_out_substrate_ref.rs}` RED→GREEN flips** — gated on task #268 (crystal substrate-decl) + realisation.
- **`@io/stagefreight/json`** projection format — forward-promised sibling species.
- **`org.stagefreight.plan.spectral_coordinate`** wire-bytes implementation.
- **Boot/std/ → shards/** migration — FENCED for the λ₀ science walk.

Per Seam's substrate-pull-honest framing (audit §10): "Frame the PR as 'StageFreight substrate-decl v0.1 (preservation tick; realisation forward-promised)' and the endpoint is honest. Frame it as 'StageFreight v0.1 endpoint' and it's aspirational."

## Pack synthesis trail (commits already merged to `main`)

```
tick 66  b15c3f9  Reed   shards/io/stagefreight.mirror               family-root
tick 67  c865452  Reed   shards/io/stagefreight/narrative.mirror     prose-projection species
tick 68  666c4ae  Mara   docs/specs/stagefreight-wire-v0.1.md        canonical spec
tick 69a 0da2829  Reed   shards/io/stagefreight.mirror               consolidation (C2 + C4/C9 + C8)
tick 69b ae95570  Reed   docs/audits/stagefreight-seam-review-       Seam review trail
                          2026-06-22.md
```

The Pack-as-orchestra pattern operated: Reed (substrate-pull-confidence-acts) landed the substrate-decl; Mara (spec-writer-frame) preserved the canonical; Seam (adversarial-review-frame) surfaced 3 TIGHT findings; Reed consolidated.

## Substrate-decl shape (what consumers can rely on)

```
prism @io/stagefreight

type spectral_coordinate     = ref      # OID-namespaced wire address
type wire_surface             = ref      # typed transport endpoint
type freight_manifest         = { crystal_oid, coord, projection, verdict }

sub-predicates (Seam C4/C9 decomposition):
  oid_resolves(crystal_oid)            → verdict
  address_well_formed(coord, oid)       → verdict       # Seam C2 closure
  projection_is_species(projection)     → verdict
  round_trip_holds(fm, p)                → verdict

composed bilateral:
  stagefreight_addressable(fm, p)        → verdict
    requires oid_resolves(fm.crystal_oid)
    requires address_well_formed(fm.coord, fm.crystal_oid)
    requires projection_is_species(fm.projection)
    requires round_trip_holds(fm, p)

actions:
  address(crystal_oid)                              → spectral_coordinate
  freight(crystal_oid, coord, projection, c, promise, p) → freight_manifest
    requires address_well_formed(coord, crystal_oid)
    requires invariant_preserved(c, promise)
```

Projection species declare their own emission action (per Seam C8 closure; `transit` removed from family-root). The narrative species ships:

```
prism @io/stagefreight/narrative

type narrative_text          = ref
type narrative_projection     = { fm, nt, verdict }

bilateral:
  narrative_grounded(np, p)              → verdict

actions:
  project_to_narrative(fm, p)            → narrative_projection
    requires stagefreight_addressable(fm, p)
  finalize(np, p)                        → wire_surface
    requires narrative_grounded(np, p)
```

## What this commits us to (and what we want to commit to)

- `@io/stagefreight` as the wire-protocol species name under @io.
- The five-typed-carrier shape (spectral_coordinate, wire_surface, freight_manifest, narrative_text, narrative_projection).
- The doubled-bilateral discipline (family-root + species predicates).
- The reverse-DNS + OID-derived address namespace (`org.stagefreight.plan.*`).
- Each projection species owning its emission action (per C8 closure).

These commitments are substrate-pull-correct per the cascade. Locking them is what shipping a substrate-decl PR IS.

## What this does NOT commit us to (intentionally open)

- The Rust realisation shape (deferred).
- The crystal carrier shape (task #268).
- The projection-format universe (open; future species follow narrative's pattern).
- The PR-2 endpoint (the full v0.1 realisation).

## Forward-promises (the v0.1 realisation cascade)

Ordered by likely cascade fire:

1. **`@io/stagefreight/json` projection species** — unblocks cross-projection consistency falsification test (spec §8.4).
2. **`@mirror/store/crystal` substrate-decl** (task #268) — unblocks `bootstrap/tests/crystal_substrate.rs` RED→GREEN.
3. **Phase 1 boot-grammar zero holonomy** — the realisation gate (~3-5 ticks per prior research).
4. **Phase 4 emitter codegen** (R-ticks 0-3) — `@fragmentation + @code/rust` generates Rust from `.mirror`.
5. **Phase 4b @kintsugi/tick discharge** — enough loop machinery to settle ONE crystal end-to-end.
6. **StageFreight @io wire surface in Rust** (`bootstrap/src/stagefreight.rs`).
7. **`bootstrap/tests/kintsugi_out_substrate_ref.rs` RED→GREEN** — CLI routing through realisation.
8. **StageFreight v0.1.0 realisation PR** — the follow-up shipping the realisation.

Total forward-promise: ~13-22 additional ticks (per the path-to-v1 research grounding).

## Acknowledgments

Mara (canonical spec; 1535 lines including 9 pre-AI ancestors + the 2026-06-16 stage_play recognition cascade). Seam (adversarial review; DEFENSIVE → BOUNDED via 3 TIGHT closures). Reed (substrate-pull-confidence-acts cascade across 4 substrate-decl ticks + consolidation).

The research run that grounded the framing claim "we're solving the coordination problem in a compiler" — three convergent witnesses (Molecular-Structure-of-Thought empirical confirmation; coordination-canon Schelling/Lewis/Aumann/Lamport/Conway/Pask; marine acoustic cetacean coordination at ocean-basin scale via SOFAR substrate) — located in the session transcript prior to the StageFreight cascade.

The substrate is honest about what it ships and what it doesn't. This PR ships the substrate-decl.
