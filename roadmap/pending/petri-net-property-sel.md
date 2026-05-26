# `@mirror/property` Petri-Net Topologies — SEL Enforcement at au + io

**Task:** #103. **Pending substrate.** Source: `~/dev/systemic.engineering/LICENSE-v1.1-draft.md` + `license/SEL.md` Part II.

## The recognition

When verified `au` (Fate's verification output type) crosses the glass wall via `@io`, the SEL terms apply. The `@mirror/property` layer enforces SEL via **petri-net topology analysis** of the system's dataflow graph.

The substrate's contribution: SEL is not enforced by lawyers at runtime; it's enforced by the substrate at compile-time + runtime via structural pattern detection on the graph. The license's §5.5(b) refers to this as the "runtime enforcement infrastructure including the Petri Net analysis layer."

## Why au + io triggers checks

- `au` is the output type of Fate inference (verified value; conductivity in context)
- `@io` is the only legitimate non-mirror surface per `@epistemologic/property/glass_wall`
- When `au` crosses `@io`, verified-value semantics meet external-world semantics
- This is the structural choke point where SEL constraints attach
- The petri net evaluates the COMPOSITION's structural pattern; matched prohibited signatures → the substrate refuses

## Detection signatures (from LICENSE v1.1 draft)

Structural patterns on the dataflow graph; not content judgments.

### §3.1.4 — Labor-input protections

- **Provenance absence** — training data ingested without worker-attributable consent record
- **Intermediary-only attribution** — provenance terminates at platform identifier, not worker
- **Withdrawal path absence** — no callable interface for labor-input contributor to remove their contribution
- **Compensation floor** — consent records below jurisdictional floor; no wage attestation
- **Post-deployment loop** — continued improvement loops ingesting new labor inputs without re-checking consent

### §3.6 — Don't Weaponize

- **Kill-chain dataflow** — Covered System output flows into weapons-control / target-selection; API 48/51/57 compliance not demonstrable
- **Mass-surveillance signature** — biometric/behavioral/communications ingest from populations under military occupation without consent provenance
- **Predictive-policing / detention-targeting signature** — classifier output flowing to policing/detention/pretrial risk; training distribution intersects discrimination axes
- **Family-separation signature** — immigration-decision dataflow with no human-review checkpoint; outputs include detention/separation/deportation of minors
- **Dissident-targeting signature** — identification based on protest/organizing/journalism/HR-defense work; output flows to state security

### §3.6.7 — Anti-Occupation

- **Occupied-territory deployment** — geographic metadata overlaps territory classified as under military occupation; no FPIC or comparable consent record
- **Indigenous-lands deployment** — geographic metadata overlaps indigenous title lands; no FPIC from indigenous governance structure

## Composition with existing substrate

- `@epistemologic/property` (existing) — verdict-valued claims; the type system that carries property checks
- `@epistemologic/property/glass_wall` (#79 ✅) — names the @io boundary
- `@kintsugi/cross_wall` (#80 pending) — substrate-pull at the @io boundary
- `au` (Fate verification output) — the type that triggers the check when it crosses @io
- Petri-net topology analysis — runtime + compile-time graph analysis

## Implementation altitude

This is `@mirror/property` substrate. Not bootstrap Rust. The petri-net rules are themselves declared in mirror grammar; the substrate self-checks via the same property verification machinery as `halts` or `glass_wall`.

## Forking architecture (LICENSE §5.5)

The §5.5 forking clause is itself a `@mirror/property`-typed constraint:

- Forks that materially weaken §3 protections trigger immediate violation
- Removing/disabling the Petri Net analysis layer triggers immediate violation
- The protection propagates with the Work (the property travels with the fork)

This makes the SEL viral at the substrate altitude. Property-level enforcement of the license's structural commitments.

## Multi-jurisdictional validity (LICENSE §8.2)

The petri-net analyzer doesn't care about the violator's nominal jurisdiction:

- License attaches to violator presence (operations, business, assets) via GDPR Article 3 targeting model
- Universal-jurisdiction grounds preserved for international-crime overlap
- The substrate's content-addressing + cryptographic verification means the audit trail is portable across jurisdictions

## Scope when implementation lands

1. **`@mirror/property/sel`** glass declaring the Petri Net analyzer interface
2. **Detection signature instances** — one substrate file per signature
3. **`au + io` composition check** at the property-verification altitude
4. **Forking-clause property** propagates with derived works
5. **Multi-jurisdictional metadata** carries with crystal provenance
6. **Compile-time + runtime evaluation** via the same machinery as `halts` / `glass_wall`

## Dependencies

- `@epistemologic/property` substrate (existing)
- `@epistemologic/property/glass_wall` (#79 ✅)
- `@kintsugi/cross_wall` (#80 pending)
- `au` type from Fate (existing in the substrate)
- Petri-net analysis primitive (proposed; would be a new substrate primitive)
- LICENSE v1.1 finalization (currently draft; Alex authoring)

## Demand signal

Activates when:
- LICENSE v1.1 finalizes (Alex drives)
- A Covered System needs to deploy with SEL enforcement live (likely v1.0 launch tier)
- An au-typed value gets routed through @io in a context where SEL evaluation matters
