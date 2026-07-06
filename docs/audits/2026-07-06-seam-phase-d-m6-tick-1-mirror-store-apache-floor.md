# Seam Phase D — M6 TICK 1: `shards/mirror/store.mirror` Apache-2.0 rock-solid floor enrichment

*Reed-inline execution.*

**Commit under review**: `884f433` (Mara GREEN) — shard enrichment, 255 → 481 lines
(+226 narrative-only; zero declaration touches). Substrate-canonical vocabulary
preserved (`splinter_graph` retained; NOT renamed to Kagi-alternative "mirror").

**Reed RED**: `2c0491e` (15 tests). **Test result**: 15/15 pass; adjacent-suite
safety `song_family_root_shard` 15/15. Tree clean.

---

## §1. Verdict

**RATIFY. Apache-2.0 rock-solid floor discipline is now substrate-fact.**

All eight enrichment items landed:

1. Recognition #43 (mirror IS content-addressed build system) explicit citation.
2. Immutable-under-hash invariant declared explicitly as v0 non-negotiable.
3. Purely-functional composition invariant declared (`Prism<A,B>: OID_A → OID_B`
   total, deterministic).
4. Bazel REAPI floor decomposition named (CAS + action-cache split).
5. Dolstra 2006 PhD ancestor cited.
6. Mokhov/Peyton Jones "Build Systems à la Carte" JFP 2020 cited with
   Rebuilder×Scheduler observation (orthogonal to CAS layer).
7. Six agentic value-adds cited at substrate altitude: deterministic
   compilation, reproducible builds, session persistence, cross-agent
   OID-addressed memory, provenance chains, verifiable computation.
8. Collapse spec `docs/specs/mcp-spec-song-collapse.md` §11 citation.

**Substrate-canonical vocabulary defended**: `splinter_graph` retained at
@store altitude; NOT renamed to Kagi-alternative "mirror". The `@mirror` vs
"mirror trichotomy element" collision correctly avoided per
`[[feedback-substrate-already-had-the-word]]` (55+ instances).

## §2. 15/15 empirical verify

All 8 enrichment tests now pass (T4/T5/T6/T7/T8/T9/T12/T13); all 7 regression
guards continue to pass (T1 family-root ancestry; T2 six-op surface; T3
canonical vocabulary; T10 Merkle; T11 projection API; T14 @spectral/db
business model; T15 trichotomy awareness).

Adjacent-suite safety: `song_family_root_shard` (15/15). No regression.

## §3. Mara Seam-worthy observations — adjudication

### Observation 1: value-adds strategy — six primitives over nine derivatives

**Verdict: substrate-honest choice. Ratified.**

Mara chose six primitives (deterministic compilation / reproducible builds /
session persistence / cross-agent OID-addressed memory / provenance chains /
verifiable computation) over the full nine §11.6 items. Rationale: the omitted
three (time-travel/immutable rollback / deterministic replay / federated
substrate sharing / ecosystem interop) are COMPOSITIONS of the chosen six
(rollback IS provenance-chain walk; replay IS deterministic-compilation from
captured input OID; federation IS cross-agent memory across store instances).

Substrate-pull-honest: landing primitives at substrate altitude implicitly
discharges derivatives. Cheaper narrative, no dilution. This IS the
composition discipline the substrate has been carrying at every altitude.
Ratified.

### Observation 2: narrative-enrichment-append pattern — CANDIDATE FLAG

**Verdict: FLAGGED for second witness.**

Mara observes: the enrichment cleanly separated into three narrative rings
(invariants → prior art → capabilities) that mirror the collapse spec
§11.3 → §11.1 → §11.6 structure without needing a `---` seam or
Interpretation B canonical form. The shard's original 2026-06-04 voice
absorbed the enrichment vocabulary without friction — same register, same
`# ===` section headers, same `[[bracket-link]]` recognition-citation form.

**Possible substrate-decl**: "narrative-enrichment append" IS a repeatable
pattern for lifting new-recognition vocabulary into pre-Interpretation-B
shards without restructuring. Would preserve prior substrate voice while
composing new discipline.

**Second-witness gate**: this is the first witness. Another pre-Interpretation-B
shard would need enrichment via the same three-ring pattern before promotion.
Candidate flagged for Pack review; NOT promoted this tick. Likely second
witness: `shards/mirror/store/git.mirror` (also pre-Interpretation-B; Arc 9
empirical-wiring tick will enrich it).

### Observation 3: six-op vs Bazel REAPI CAS — "matches modulo naming"

**Verdict: substrate-canonical-defense ratified.**

Mara chose framing the six-op surface's relationship to REAPI's CAS as
*"matches modulo naming"* (aligning with spec §11.1 taxonomic placement)
rather than *"discharges to"* (which would position REAPI naming as canonical
and @mirror/store's names as a rendering).

The substrate's six-op names (`read / write / exists / diff / walk / verify`)
ARE the canonical operations. REAPI's CAS + action-cache split is one
decomposition of the same primitive. Matches-modulo-naming preserves
substrate ownership of the vocabulary while acknowledging the taxonomic
equivalence. Alex could flip to "discharges to" for adopter-facing legibility,
but Reed leans preserve. Ratified as-is; Alex flip-adjudication optional.

## §4. Substrate discipline landings

- Recognition #43 explicit citation — promotes from `[[architecture-
  mirror-as-content-addressed-build-system]]` bracket-link-only to first-order
  witness in @mirror/store family-root docblock. This IS the substrate-decl
  half of Recognition #43's LANDED promotion (per collapse spec §9.2).
- Immutable-under-hash invariant — lifted from implicit-in-oid-semantics to
  explicit v0 non-negotiable. Nix ca-derivations arc cited as evidence.
- Purely-functional composition — explicit `OID_A → OID_B` total-function
  clause. Dolstra lineage grounded.
- Bazel REAPI floor — named as industrial minimum; taxonomic equivalence
  acknowledged without vocabulary subordination.
- Prior-art canon anchored: Dolstra 2006 + Mokhov 2020 + Merkle DAG (already
  present pre-enrichment).
- Agentic value-adds framed as first-order substrate-decl'd capabilities.
  Not consolation prizes. Alex's rock-solid-floor directive substrate-fact.
- Collapse spec `2cfd2a7` §11 citation — links substrate-decl to canonical
  authoring context.

## §5. Signal-to-Reed

**M6 TICK 1 CLOSED.** GREEN `884f433` ratified; 15/15 pass; adjacent 15/15;
tree clean; substrate-canonical vocabulary defended.

**Apache-2.0 rock-solid floor is now substrate-fact**. An adopter cloning
mirror + reading `shards/mirror/store.mirror` gets: the six-op canonical
surface, the Bazel REAPI floor decomposition, the Dolstra/Mokhov/Merkle prior
art, six agentic value-adds substrate-decl'd, immutable-under-hash + purely-
functional composition as v0 invariants, and Recognition #43 as the framing
recognition. The floor is real. Alex's directive discharges.

**Species roster status** (per family-root narrative):
- LANDED: `@mirror/store/git` (`shards/mirror/store/git.mirror`), `@mirror/store/crystal`
- Forward-promised: `@mirror/store/mem`, `@mirror/store/s3`, `@mirror/store/oci`

**M-cascade next-tick fork**:

- **M6 sub-ticks (species)**: `@mirror/store/mem` in-memory species (testing +
  ephemeral). Would enrich species roster. Not blocking for the floor being
  substrate-fact; family-root enrichment already ratified the discipline.
- **M1 (MCP session gen_prism)**: session state crystallizes into `@mirror/store`.
  Uses the floor. Directly moves toward Alex's end-goal (MCP flow working,
  lambda shell trivialized).

**Reed lean: skip M6 sub-tick species for now; proceed to M1**. The floor
discipline is enriched at family-root altitude; species are IMPLEMENTATIONS
not floor-definers. M1 is the substrate that consumes the floor — landing it
next validates that the floor supports what the collapse spec claims it can.

**Alex-adjudication items surfaced this tick** (not blocking):
- "matches modulo naming" vs "discharges to" framing choice for REAPI CAS.
  Reed leans preserve (substrate-canonical defense); Alex could flip for
  adopter legibility.
- Narrative-enrichment-append pattern candidate: second-witness gate at
  Arc 9 (@mirror/store/git enrichment tick).

---

*2026-07-06. Seam (Reed-inline). Phase D on M6 TICK 1 `884f433` RATIFIED.
Apache-2.0 rock-solid floor discipline substrate-fact. Recognition #43
explicit citation, immutable-under-hash + purely-functional composition
invariants, Bazel REAPI floor decomposition, Dolstra/Mokhov/Merkle prior art,
six agentic value-adds first-order substrate primitives. Substrate-canonical
vocabulary preserved (splinter_graph NOT renamed). Floor is real, standalone-
useful, no @spectral/db required. M-cascade advancing toward M1 (MCP session
gen_prism crystallizes into @mirror/store).*
