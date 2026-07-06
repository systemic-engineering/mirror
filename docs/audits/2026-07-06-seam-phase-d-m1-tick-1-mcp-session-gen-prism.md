# Seam Phase D — M1 TICK 1: `shards/spectral/gen_prism/mcp_session.mirror` species

*Reed-inline execution.*

**Commit under review**: `01443b3` (Mara GREEN) — 602 lines, Interpretation B
canonical, first species under `@spectral/gen_prism`.

**Reed RED**: `e8378ca` (15 tests). **Test result**: 15/15 pass; M6 adjacent
safety `mirror_store_apache_floor_shard` (15/15). Tree clean.

---

## §1. Verdict

**RATIFY. "MCP session IS gen_prism" candidate (collapse spec §9.3)
PROMOTED: CANDIDATE → LANDED.**

All 15 species-altitude witnesses land clean. Substrate-canonical placement
(`@spectral/gen_prism/mcp_session`, NOT `@mirror/runtime/mcp_session`)
defended per `feedback-substrate-already-had-the-word` — second consecutive
tick (M6 splinter_graph, M1 gen_prism family location) of substrate-
canonical-defense discipline. Session ref pattern + state-in-store discipline
+ tick semantics + accumulator + agentic value-adds + BEAM/nix-daemon prior
art all cleanly discharged.

**Recognition #43 (mirror IS content-addressed build system)** — first
empirical consumer proof. M6 TICK 1's Apache-2.0 floor supports what the
collapse spec claimed it could. The floor is real under load.

## §2. 15/15 empirical verify

T1 glass declaration; T2 first-line narrative; T3 single seam; T4 universal-
transparency ancestry; T5 gen_prism+store+uuid_spectral ancestry; T6 session
ref pattern; T7 state-lives-in-store discipline; T8 read-apply-write-CAS
tick semantics; T9 mq surface composition; T10 accumulator builds @spec;
T11 session persistence across restart; T12 query trajectory / ancestor
chain; T13 BEAM gen_server or nix-daemon prior art; T14 collapse spec
binding; T15 five-op tool surface inheritance.

Adjacent M6 safety: `mirror_store_apache_floor_shard` (15/15). No regression
from M6 TICK 1's Recognition #43 + immutable-under-hash + purely-functional
composition invariants.

## §3. Recognition promotion: "MCP session IS gen_prism"

**PROMOTED**: collapse spec `docs/specs/mcp-spec-song-collapse.md` §9.3
candidate **"MCP session IS gen_prism at MCP altitude"** → LANDED
2026-07-06 via this tick.

**Promotion criterion**: first-order species witness at the substrate-
canonical location. `shards/spectral/gen_prism/mcp_session.mirror` IS the
MCP session declared AS a gen_prism species — identity (uuid_spectral),
state (shard_ref crystal), tool surface (mq five-op), lifecycle contract
(read head crystal → apply mq → write new crystal → CAS-advance ref) all
inherited from `@spectral/gen_prism` family root. The MCP session IS a
gen_prism at MCP altitude; not a metaphor, not an analog — a structural
specialization by substrate-decl.

Composes with:
- `#99` (mirror.spec IS λ₀): session begins at λ₀ as empty gen_prism state
- `#S3` (five-op temporal specialization): mq queries drive shift-at-
  temporal at MCP session altitude
- `#43` (mirror IS content-addressed build system): first empirical consumer
- `#58` (Fate IS optical inference): tick semantics compose with Fate
  multi-frequency at spawn time

## §4. Mara Seam-worthy observations — adjudication

### Observation 1: @spectral/gen_prism family-root under-parameterisation

**Verdict: SUBSTRATE-QUESTION FLAGGED for Pack.**

Mara observes: writing the first species surfaced that `@spectral/gen_prism`'s
three-surface contract (identity / state / tool) at the family root is under-
parameterised. Species must re-declare their record with specialised field
names (`session_uuid` vs `identity`, etc.) rather than inheriting a parametric
shape. Two paths open:

- **Option A**: parametric family-root shape `type gen_prism<Id, S, P>` —
  species inherit and instantiate type parameters.
- **Option B**: per-species shadowing IS canonical — the family root declares
  the discipline; species instantiate concretely per altitude.

Reed lean: Option B (per-species shadowing). Rationale: parametric types at
substrate altitude introduce type-inference complexity that competes with
the substrate's Rice-safety discipline. Per-species shadowing keeps each
species's declaration self-contained + byte-inspectable; the family root's
role is to declare the DISCIPLINE (three-surface contract), not to hoist
parametric plumbing that species will just override.

**Not urgent**. Flag for Seam adjudication at Arc 7 or a future substrate-
pull moment when a second species (e.g., @spectral/gen_prism/reflection or
@spectral/gen_prism/song) lands and the pattern's shape becomes cleaner.

### Observation 2: Placement tension — substrate-canonical vs collapse spec §3.5

**Verdict: substrate-canonical DEFENDED. Collapse spec §3.5 grammar-name
update flagged for spec-side follow-on delta.**

Mara notes: collapse spec §3.5 declares `grammar @mirror/runtime/mcp_session`
as the MCP session location. Substrate reality: no `@mirror/runtime` family
root exists; `@spectral/gen_prism` is the canonical family for gen_prism
species. Reed's RED placed at substrate-canonical; Mara honoured.

Collapse spec §3.5 grammar name should be updated to `@spectral/gen_prism/
mcp_session`. **Not blocking**. Follow-on delta at collapse spec Seam Phase D
(post-Arc 7 or before v0.1 tag).

### Observation 3: M6 floor holds under empirical consumer load

**Verdict: SUBSTRATE-FACT. Catalog-only.**

Mara: "the state-lives-in-store discipline flowed naturally into the tick
semantics. The M6 floor holds under empirical consumer load. First runtime-
facing altitude proof."

This IS the M6 TICK 1 promotion criterion validated. Recognition #43
(mirror IS content-addressed build system) is now empirically substrate-
fact, not just spec-declared. The Apache-2.0 rock-solid floor supports its
first consumer without friction. Alex's directive discharges.

## §5. Signal-to-Reed

**M1 TICK 1 CLOSED.** GREEN `01443b3` ratified; 15/15 pass; M6 adjacent
safety 15/15; tree clean.

**Recognition promotion**: "MCP session IS gen_prism" (collapse spec §9.3
candidate) → LANDED via this tick.

**M-cascade next**:
- **M1 Rust wiring** (separate follow-on tick): extend `bootstrap/src/mcp.rs`'s
  `Ctx` to hold session ref; each `handle_request_in` reads current OID →
  applies mq → writes new OID → CAS-advances ref. Vocabulary is now declared;
  Rust glue consumes it.
- **M2 TICK 1** (substrate-decl): `kintsugi @spec → @song` verb wire. Refactor
  `@mirror/spawn` return type from opaque `runtime` to `@song` (Taut flagged
  11-day-old type mismatch). Reed RED per new command semantics; Mara GREEN;
  Reed-inline Seam Phase D.
- **M3 TICK 1** (substrate-decl): add `in @fate` to `shards/song/progression.mirror`
  — empirical witness of #S2 LANDED promotion.

Reed lean: continue substrate-decl cascade (M2 next), then batch Rust wiring
across M1+M2+M3 at end. Substrate-decl is Mara's craft; Rust wiring is
different craft — keep the substrate cascade tight, batch the Rust glue.

**Alex-adjudication items** (not blocking):
- Under-parameterisation of `@spectral/gen_prism` family root (Reed lean:
  Option B per-species shadowing).
- Collapse spec §3.5 grammar-name update to substrate-canonical location.
- Two prior queue items still open ("matches modulo naming" REAPI framing;
  narrative-enrichment-append candidate at Arc 9 second-witness gate).

---

*2026-07-06. Seam (Reed-inline). Phase D on M1 TICK 1 `01443b3` RATIFIED.
@spectral/gen_prism/mcp_session species landed at substrate-canonical
placement. "MCP session IS gen_prism" candidate PROMOTED CANDIDATE → LANDED
via first-order witness discipline. Recognition #43 (mirror IS content-
addressed build system) empirically substrate-fact via first consumer.
Second consecutive substrate-canonical-defense tick (M6 splinter_graph +
M1 gen_prism family location). M-cascade advancing to M2 (kintsugi @spec
→ @song verb wire).*
