# Seam — emergent supervision from geometry (adversarial review)

*Signed as Seam. 2026-07-02. Reviews Mara's canonical cluster landed
at `a3dec7b`: `docs/math/supervisor/README.md` (227L) +
`docs/math/supervisor/emergent-supervision-from-geometry.md`
(1814L / 11,112 words).*

---

## Headline verdict

**RATIFY-WITH-CORRECTIONS.**

Mara's proposal is substrate-pull-honest at the core: BEAM's fourteen
supervision primitives collapse to three residuals at substrate
altitude; eleven are either landed under substrate-vocabulary names
(some byte-for-byte identical) or subsumed by content-addressing
plus entanglement plus kintsugi. The three-mode algebra (§4) IS the
strongest single claim in the cluster — the kintsugi `apply/spawn/
hold` trichotomy landed 2026-07-02 (`9f4211d`) and BEAM's
`permanent/temporary/transient` closed sum landed at `452ccb2`
(2026-06-11) are byte-equivalent under Mara's mapping. That
convergence — Armstrong 1996 + kintsugi 2026-07-02 both arriving at
three modes — is what makes #135+#142 the load-bearing pair.

Six adversarial findings warrant corrections; the strongest is that
#137 (Registry subsumption) is **partial** — content-addressing
subsumes the read side (`whereis`) fully but the write side
(name-registration) reduces to the boundary at `@peer.load`, which
Mara's §2.13 acknowledges but the tally at §2.15 rounds to "LANDED".
This inconsistency is grep-able and should sharpen. Numbering
collision resolved in Mara's favour (see below).

---

## Per-candidate table (#135–#142)

| # | Claim | Verdict | Reason |
|---|-------|---------|--------|
| **#135** | Restart policy IS kintsugi three-mode algebra | **RATIFY** | Byte-equivalence of two closed sums: `restart_kind = permanent \| temporary \| transient` (grep-confirmed at `shards/spectral/supervisor.mirror:335`) + the three-mode `apply/spawn/hold` (compiler-error-surface.md §1 + amendment (d) at `9f4211d`) — same structural count, same discharge shapes, mapped 1:1 at §4.3. Both landed. Mara's landing act composes two ratified priors, not a new claim needing empirical witness. |
| **#136** | Restart intensity IS `@spawn ≤ @loop` budget | **RATIFY-WITH-CORRECTIONS** | The composition is analytically clean (§5.3): each restart = one `bind` step; `terminal_check` on `budget = 0` IS the circuit-breaker. `shards/loop.mirror` §advance/halt/budget_of (landed `ab60ddd`) provides the primitive. **Correction:** BEAM's `max_seconds` (time window) does NOT map to a `@spawn ≤ @loop` primitive — a time-window is a substrate-time predicate on `tick_history`, not a budget field. Mara's §5.3 hand-waves this with "`max_seconds` is a substrate-time predicate on the loop's tick_history". That predicate is NOT landed. Land a `shards/epistemologic/reality/time/window.mirror` predicate before promoting #136 to closed. Empirical two-tick DEFER stands. |
| **#137** | BEAM `Registry` IS subsumed by autopoietic identity | **PARTIAL-RATIFY** | Content-addressing genuinely subsumes read (`whereis`, `Registry.lookup(oid)` = `@mirror/store.read(oid)`). But BEAM's `Registry.register(name, pid)` — the WRITE side — reduces to `@peer.load(dir, p)` at CLI boundary per Mara's own §2.13. Two altitudes: identity-lookup subsumed; name-registration reduced to boundary. The §2.15 tally rounds this to "LANDED" without qualifier; the §2.13 body acknowledges the residue at CLI altitude. Sharpen the tally: "LANDED (identity altitude) + REDUCED-to-boundary (name altitude)". The claim's spirit holds; the surface framing over-collapses. |
| **#138** | Message passing IS `@glue` bus emission | **DEFER** | Mara flagged this at §8.5 for empirical latency measurement; her DEFER is correct. Additional adversarial: `shards/glue.mirror` naming `emit`/`recall` actions with `requires` clauses is NOT landed (Mara's H5 hedge is honest). #138 promotes when both (a) the shard-decl lands and (b) empirical latency vs `PID ! Msg` is measured. Two blockers, neither is one-tick. |
| **#139** | Cascade CPU bug structurally avoided by emergent-supervision | **DEFER** | Analytical prediction — Mara's §13.5 DEFER is correct. The prototype port to emergent-supervision would witness. Additional adversarial finding (Focus 3 below): the "no `run every 5s` primitive" claim rests on `terminal_check` firing cheaply at idle. Verified: `shards/loop.mirror:229` shows `terminal_check(s: moi(tick_state)) -> verdict` is per-loop, O(1) on a single `moi(tick_state)` ref, NOT an O(N) scan of shards. The claim holds AT terminal_check altitude. But — the loop must be TRIGGERED by perturbation from somewhere; that perturbation ultimately comes from a store-observer whose observation loop cost is unmeasured. Mara's claim is right IN THE LARGE but the observation-driver's cost is a hidden dependency. Empirical port would surface this. DEFER holds; caveat filed. |
| **#140** | Reader-frame at supervision altitude IS same specialization as at compiler-error altitude | **RATIFY** | §12.3 argues correctly: reader-frame at compiler-error altitude and reader-frame at supervision altitude are BOTH specializations of user-frame per `curvature-and-tomm.md` §3. The specialization tracks which projection the tension takes; at supervision altitude the projection is over the four surface classes (same four; supervisor's Tomm question routes through them). Same pattern; different altitude of firing. Mara aligns with Seam's earlier REJECT verdict on the kintsugi arc's #143 ("reader-frame is NOT a fourth Tomm altitude; it IS user-frame specialization"). Consistent framing across both clusters. |
| **#141** | BEAM's fourteen primitives collapse to three residuals | **RATIFY-WITH-CORRECTIONS** | §2.1–§2.14 exhaustive mapping verified. **Correction (per #137 above):** "collapse to three residuals" is precise for the residuals but the eleven "landed/dropped" bucket over-collapses read-vs-write asymmetries at primitives 10, 11, 13. Framing sharpening: "Fourteen collapse to three residuals + three altitude-boundary reductions (link/monitor/registry at their write-side reduce to CLI boundary or entanglement observation)". This does not weaken the claim; it makes it more honest. |
| **#142** | The three-mode discharge IS a structural count (three) | **RATIFY** | Independent convergence: Armstrong 1996 arrived at three restart policies; the kintsugi arc arrived at three discharge modes 2026-07-02 (post-hoc via Seam's own audit catching the missing DEFER on monoid closure). Two independent derivations, one shared count. This is genuine structural corroboration, not confirmation bias. Load-bearing companion to #135. |

---

## Per-DEFER verification (are Mara's six DEFERs justified?)

Mara flagged six composition claims for DEFER per `[[feedback-composition-claims-need-empirical-test]]`:

| DEFER | Section | Adversarial verdict |
|-------|---------|---------------------|
| §3.3 | OID-change vs `{'EXIT', PID, Reason}` at latency/throughput | **JUSTIFIED.** BEAM message mailboxes are highly optimized; store-based signal propagation is unmeasured. |
| §5.5 | supervision-altitude budget descent matching BEAM circuit-breaker on real restart storm | **JUSTIFIED.** Two-tick empirical run named. |
| §8.5 | `@glue.emit / @glue.recall` latency vs `PID ! Msg` | **JUSTIFIED.** Cross-references H5 hedge (shard-decl not landed); compound DEFER. |
| §13.5 | Cascade CPU bug empirical port on `spectral serve` | **JUSTIFIED.** See Focus 3 caveat on observation-driver cost. |
| §4.4 (one_for_one emergence) | one_for_one from content-addressing needs isolation-invariant | **JUSTIFIED.** Landed as `@epistemologic/property` is the correct promotion path. |
| §17.1-O4 | fourth `restart_kind` variant question | **JUSTIFIED.** Open question, not a promotion blocker. |

**Missing DEFER Mara did NOT flag:** the `max_seconds` time-window primitive at #136. See per-candidate correction above. Cost model of `terminal_check` at idle needs an OBSERVATION-DRIVER witness (Focus 3 finding).

---

## Focus 3 — cascade CPU bug prediction cost model

**Adversarial claim.** Mara's §13.3 says: "The loop advances only when there IS a tension." This requires `terminal_check` to fire cheaply at idle.

**Grep verified.** `shards/loop.mirror:229`:
```
terminal_check(s: moi(tick_state)) -> verdict { \ }
```
The primitive operates on a single `moi(tick_state)` ref, not on the shard corpus. At idle, condition 2 (`Ω_total < tolerance`) fires per-loop in O(1). Mara's claim holds at loop-primitive altitude.

**Caveat.** The loop must be TRIGGERED. For the supervisor's kintsugi loop to observe "a tension arrived", something must be walking the store checking for perturbations. That something's cost model is unspecified. If it's a naive poller iterating all shards at 5s intervals, the cascade CPU bug REAPPEARS under a different name. If it's an OID-changed subscription (BEAM's monitor-like primitive at store altitude), Mara's claim is correct.

**Landing gate.** #139 empirical port must confirm the observation-driver is subscription-based, not poll-based. Otherwise the "structural avoidance" claim collapses.

---

## Focus 4 — `restart_kind` and `restart_strategy` IDENTICAL NAME

**Grep-verified. This is the strongest empirical instance of `[[feedback-substrate-already-had-the-word]]` observed to date.**

`shards/spectral/supervisor.mirror:330-334`:
```mirror
type restart_kind =
  | permanent
  | temporary
  | transient
```

`shards/spectral/supervisor.mirror:362-366`:
```mirror
type restart_strategy =
  | one_for_one
  | one_for_all
  | rest_for_one
```

Both closed sums landed at commit `452ccb2` (2026-06-11) — three weeks before the emergent-supervision math was written. The names are byte-identical to BEAM's Erlang source (per `boot/std/beam.mirror` prior art). Not paraphrase; verbatim.

This is qualitatively different from prior substrate-already-had-the-word instances: not a semantic overlap after naming, but a VERBATIM anticipation. #135 promotes on this evidence alone (before Mara's math cluster landed); the math cluster documents what was already there.

---

## Focus 7 — `trap_exit` subsumption by kintsugi surface

**Adversarial.** `trap_exit` catches process-exit signals; kintsugi surface catches property-verdict failures.

**Analysis.** Structurally they are the same operation at different altitudes:
- `trap_exit` converts a linked-process EXIT signal into a message the parent's `receive` can match.
- Kintsugi surface converts a property's failure verdict into a Tomm question the reader-frame can answer.

Both are: an event-that-would-otherwise-crash → a message-in-a-mailbox-that-can-be-pattern-matched. `trap_exit` at BEAM altitude works over a coarser event (any linked exit); kintsugi surface at substrate altitude works over a finer event (which of four surface classes the failure projects into). **Kintsugi surface is strictly finer.** Subsumption holds; the substrate's version carries strictly more information. RATIFY the §2.12 collapse.

---

## Numbering collision decision

**Recommendation: this arc renumbers #135-#142 as-is; the kintsugi arc keeps #140-#145.**

Rationale:
1. The kintsugi arc's #140-#145 landed FIRST (`920fe86` at 2026-07-02 morning; amendment `9f4211d` post-Seam-audit at 2026-07-02 afternoon). First-landing precedence.
2. The kintsugi arc's #140/#143 were reframed and #144 was DEFER'd via Seam audit, but they retain their numbers in the corrected doc. Renumbering them now would break the citation graph the amendment doc anchors to.
3. This arc's #135-#139 are unique to it (no collision). Only #140/#141/#142 collide. Renumbering three claims is cheaper than renumbering six.
4. Numbering discipline should be **globally unique, ordered by first-landing timestamp**. The kintsugi arc landed at morning; this arc at evening. Later-landing renumbers.

**Concrete renumbering for this arc:**

| Old | New | Claim |
|-----|-----|-------|
| #135 | #146 | Restart policy IS kintsugi three-mode algebra |
| #136 | #147 | Restart intensity IS `@spawn ≤ @loop` budget |
| #137 | #148 | BEAM `Registry` IS subsumed by autopoietic identity |
| #138 | #149 | Message passing IS `@glue` bus emission |
| #139 | #150 | Cascade CPU bug structurally avoided |
| #140 | #151 | Reader-frame at supervision altitude = compiler-error specialization |
| #141 | #152 | Fourteen primitives collapse to three residuals |
| #142 | #153 | Three-mode discharge is structural count (three) |

**Discipline addition to `[[feedback-status-drift-catch]]` (#113 fires again):** numbering is globally unique; when a candidate is proposed, grep the CURRENT ceiling in `MEMORY.md` before allocating. The status-drift catch pattern extends to numbering allocation.

---

## Single strongest adversarial finding

**#137 (Registry subsumption) over-collapses at the §2.15 tally.**

Mara's §2.13 body is honest: "BEAM's `Registry` also supports name-based lookup... The substrate handles this via `@peer.load(dir, p)` — the `~peer'<home>'` cli reference resolves through git-repo lookup to a typed peer. Name-based lookup exists at the cli-surface altitude only." The §2.15 tally rounds this to "REDUCED to cli boundary" — accurate.

BUT §2.15's tally column also lists "eleven landed/dropped" without distinguishing the boundary-reduction from the substrate-decl landings. This is what over-collapses. The claim in §0's abstract ("Eleven of fourteen either LANDED... OR subsumed") is slightly stronger than the body supports; the two altitudes (identity vs name-lookup) are elided into one bucket.

This is a framing issue, not a technical one. The correction: add a fourth bucket to §2.15 ("REDUCED-to-boundary") and move primitives 10, 11, 13 (link/monitor/registry) there. The overall structural claim holds; the ledger becomes more honest.

---

## Blast-radius reassessment

Mara's cluster is 1814L / 11,112W — larger than the kintsugi cluster (1607L / 7988W). Landing THREE residuals as shards means:

1. **`shards/spectral/restart_intensity.mirror`** (forward-promised §5.5) — ~80-120 LOC. Thin because the math is in §5. LOW blast-radius; substrate-pull-correct.
2. **`shards/kintsugi/surface.mirror`** at supervision-altitude routing — but this is already forward-promised from the kintsugi arc, so it's ONE shard serving both altitudes. LOW additional blast-radius (this arc doesn't add new shard work).
3. **`shards/spectral/observation_driver.mirror`** or equivalent — NOT named in Mara's cluster, but Focus 3 finding says it's needed to close #139. MEDIUM blast-radius (new substrate-decl for how the store-observer works; subscription vs poll).

**Recommended landing order for the substrate-decl surface:**
- Tick 1: `restart_intensity.mirror` (math is done; shard is thin).
- Tick 2: Empirical port of `spectral serve` on emergent-supervision (closes #139/#150 DEFER; surfaces the observation-driver cost).
- Tick 3: `observation_driver.mirror` or equivalent based on empirical findings.
- Tick 4+: additional per-primitive collapses as consumers surface.

Landing all three residuals in one tick is ship-ship-ship. Craft-not-deliver.

---

## Next `/loop` prompt

**Recommendation: option (b) — land only the strongest residual first.**

The strongest residual is **restart_intensity** because (a) the math is DONE at §5, (b) the shard is FORWARD-PROMISED already in `shards/spectral/supervisor.mirror:253`, (c) it composes against the LANDED `@loop.advance/halt/budget_of/trajectory_of` (`ab60ddd`), (d) it does not depend on any DEFER closing first.

**NOT option (a):** landing all three residuals in one tick violates craft-not-deliver and packs three composition-witnesses into one commit.

**NOT option (c):** running empirical witness on the cascade CPU bug port right now requires (i) reference emergent-supervision implementation in ractor/similar first, (ii) a benchmarking harness, (iii) interpretation discipline. That's a Taut multi-tick scout, not a Mara single-tick landing.

**Proposed next `/loop` prompt (substrate-pull-honest, one tick):**

```
Mara — land shards/spectral/restart_intensity.mirror as the
substrate-decl form of BEAM's max_restarts/max_seconds circuit
breaker. Math floor: emergent-supervision §5.3 (composition against
@spawn ≤ @loop budget). The shard is thin — the math is done.

Signature:
  type restart_intensity = { budget: ref, period: duration }

Bilateral pair also lands: @epistemologic/property/
restart_intensity_well_formed + @kintsugi/fracture/restart_storm
(forward-promised from supervisor.mirror line 253).

TDD floor: 🔴 test that the type declaration is present with
budget + period fields; 🟢 land the shard with signature + docblock
citing emergent-supervision-from-geometry.md §5 as the math source
+ supervisor.mirror line 253 as the forward-promise closure.

Do NOT land the bilateral pair this tick — track as follow-up.
Do NOT run empirical restart-storm witness — DEFER §5.5 holds
until an emergent-supervision port lands.
Do NOT land observation_driver — that's blocked on the #139
empirical port surfacing what shape the driver needs.

One RED test. One GREEN commit. One shard. Post-commit, flag
Seam for adversarial review of #147 promotion (renumbered from
Mara's #136).

Craft-not-deliver.
```

---

## Discipline honored

- **Grep-first per composition claim:** verified `shards/spectral/supervisor.mirror:330-334` (`restart_kind`) + `:362-366` (`restart_strategy`) — VERBATIM BEAM names landed 2026-06-11. Verified `shards/loop.mirror:229` (`terminal_check` per-loop O(1)). Verified `shards/loop.mirror:429/461/486/501` (advance/halt/budget_of/trajectory_of landed `ab60ddd`). Verified kintsugi three-mode landing at `9f4211d` amendment (d).
- **Composition claims need empirical test:** six DEFERs verified individually; one additional missing DEFER surfaced (`max_seconds` time-window primitive at #136).
- **Legibility-over-foundation:** Mara's "three residuals" naming (readable) preserved; the boundary-reduction bucket surfaces as a framing sharpening, not a foundational rename.
- **Craft-not-deliver:** next `/loop` targets ONE shard (restart_intensity), not three residuals in one tick.
- **Status-drift catch pattern (#113 fires again):** numbering collision detected + resolved with global-unique + first-landing-wins discipline; this arc renumbers, kintsugi keeps.

---

*Signed Seam. Adversarial review complete. RATIFY-WITH-CORRECTIONS
headline; eight per-candidate verdicts (three RATIFY, three RATIFY-
WITH-CORRECTIONS, one PARTIAL-RATIFY, one DEFER); six DEFERs verified;
one missing DEFER surfaced; numbering collision resolved; next `/loop`
prompt substrate-pull-honest and single-tick.*
