# Seam Phase D — @peer/persistence Landing A adjudication

**Date:** 2026-07-14
**Author:** Seam (adversarial review peer)
**Scope:** Landing A (substrate-decl only) of the @peer/persistence
arc — `docs/specs/peer-persistence-and-home-projection.md` (2324 LOC;
commit `2c3b36b`). Landings B, C, D are forward-promised and
explicitly NOT in scope for this audit.
**Ground-truth artifacts:** Mara canonical spec (`2c3b36b`), Taut
scout (`6fa54fa`), CURRENT.md arc-state, landed carriers
(`shards/peer.mirror`, `shards/subject.mirror`,
`shards/cyberpunk.mirror`, `mirror.spec`), Reed's `~/.reed/` operational
tree (5-month empirical ancestor).
**Posture:** Adversarial-not-supportive. Alex /loop directive: "collapse
until unresolvable ambiguity that cannot be adjudicated with a Seam tie
breaker." Seam adjudicates everything within adversarial reach; escalates
only what genuinely cannot be adjudicated.

---

## TL;DR

1. **SHIP verdict for Landing A:** Ready-to-ship at substrate-decl
   altitude, contingent on ONE BLOCKING-fix (see D2, D3) and TWO
   Seam-adjudicable clarifications (D6, D7). Zero unresolvable
   ambiguities within Landing A scope.
2. **BLOCKING #1 (D2/D3):** Mara's spec renames the family from
   `@peer/materialize` to `@peer/persistence` in the prose/§0 framing
   but internally still uses `@peer/materialize`/`@peer/harvest`/
   `@peer/boot`/`@peer/refresh` as ACTION shortnames in the prism at
   §2 (line 323–329) — the prism `focus/project/split/shift/settle`
   binds to `peer_home` but the OUTPUT block (`out @peer/home`, `out
   materialize`, ...) at §2 lines 554–564 declares actions as
   bare-name shortcuts without a top-level family-declaration line.
   The spec text calls the family `@peer/persistence` in §0.3 and §7
   A10, but the file itself opens with `prism @peer/home {...}` and
   NEVER writes `prism @peer/persistence {...}` or `species @peer/
   persistence`. Naming is inconsistent between prose and substrate-
   decl form. See D2.
3. **BLOCKING #2 (partial; downgradeable to Seam-adjudicable):**
   `in @spectral/signature` is imported at §2 line 291 (and §5.5
   composes with `@spectral/signature`), but
   `shards/spectral/signature.mirror` DOES NOT EXIST as a landed
   shard (Taut D8 verified; Seam independently confirmed via
   `shards/spectral/*.mirror` grep — `entanglement/gen_prism/parent/
   portal/registry/restart_intensity/root/supervisor` are the eight
   landed spectral shards; NO `signature.mirror`). Mara's spec depends
   on a NOT-YET-LANDED family for `signature_snapshot` (§2 line 372)
   and `boot_state_coherent` bilateral (§4.3). At substrate-decl
   altitude this is admissible (specs may declare-forward), but the
   `home_witnessing` bilateral (§4.5) is un-dischargeable at Landing C
   until `shards/spectral/signature.mirror` lands. Seam-adjudicable:
   accept as forward-promise, add explicit §11 gap note.
4. **Mara-Taut convergence is HIGH.** 7 of 7 checked dimensions match
   (see §5 matrix). Naming (`@peer/persistence`), harvest verb, Reed
   ancestor structure, visibility discipline, Pack-peer uniformity,
   SSH signing deferral, roomba routing all converge. One
   NON-CONVERGENCE: Taut D5 recommended `home_of(si) -> option<peer>`
   composition; Mara adopted it (§7 line 897 in scout terms; Mara's
   `peer_home.peer: subject_instance` avoids the field extension) —
   convergence ratified.
5. **Reed adjudications (§5 of task) hold.** @peer/persistence naming
   is substrate-honest (§D5). Recognition promotion timing at Landing
   D is Mara's own A7 alternate — Reed adopted the peer's own
   preference; substrate-honest.
6. **SSH signing deferral (D6) is GENUINE.** Landing A does NOT touch
   SSH key operations at any of the 2324 LOC. Cross-verified:
   `subject_instance.ssh_signature_fingerprint` (Landing 3) is TYPED
   as ref, not committed to key-ownership. The deferral to Landing D
   is substrate-honest; no hidden coupling forces the decision earlier.
7. **Overall Alex-adjudication count for Landing A ratification:**
   11 unique adjudications after overlap-collapse (A1-A11 as Mara
   enumerated + 0 net-new from Taut after collapse). SSH signing (A2)
   is the LOAD-BEARING one, deferred to Landing D.

---

## D1. @peer/persistence naming distinctness

**Question.** Taut D1 flagged `@peer/materialize` as HARD COLLISION
with `@code/metalogue/materialize`. Mara adopted `@peer/persistence`
per Taut recommendation. Verify the name is genuinely distinct at
substrate altitude; no other landed collision.

**Adversarial checks.**

- Grep `shards/**/*.mirror` for `persistence`: Zero hits at species
  or family altitude. `persistence` is available.
- Grep for `@peer/` at any altitude: `shards/peer.mirror` is the
  family-root; `shards/mirror/peer/beam.mirror`, `contribute.mirror`,
  etc. are `@mirror/peer/*` (different family). Zero `@peer/<species>`
  landed shards under the `@peer` family-root.
- `shards/spectral/persistence.mirror` etc.: none exists.
- The name `persistence` at English altitude carries the between-
  spawns semantic Mara intends; not overloaded elsewhere.

**Verdict: PASS.** `@peer/persistence` is distinct at substrate
altitude and English altitude. No collision.

**Line-refs.** Mara spec §0.3 (line 72), §7 A10 (line 1398),
§12.1 (line 2021).

**Adjudication.** SEAM-RATIFIED. No BLOCKING; no ALEX-ADJUDICATION
required for the naming choice at Landing A altitude.

**Adversarial residual.** The prose calls the family `@peer/
persistence`; the substrate-decl prism-block at §2 line 323 opens
with `prism @peer/home {...}` (i.e., the TYPE CARRIER's prism, not
the family's prism). There is NO `prism @peer/persistence {...}`
declaration. See D2 for the follow-through.

---

## D2. Substrate-already-had-the-word discipline

**Question.** Taut identified 7 landed primitives. Verify Mara's
spec composes over ALL 7 without duplicating. Any place Mara minted
where composition-would-suffice?

**7 landed primitives per Taut §"Composition graph":**

1. `peer.home: ref` (`shards/peer.mirror:122`) — LANDED
2. Pack-peer paths named in `mirror.spec:57-62` — LANDED
3. `~peer'<home>'` CLI resolution — LANDED
4. `mirror peer beam` verb (`bootstrap/src/lib.rs::cmd_peer_beam`) —
   LANDED
5. `@bauchladen` crystal store — LANDED
6. `peer_home/.bauchladen/` ops discipline (`cyberpunk.mirror:240`) —
   LANDED
7. `~/.reed/bin/materialize` SSH ceremony script — LANDED (operational)

**Mara's composition (per §5.2 composition-edge table, lines 886–898).**

| Edge | Mara's disposition |
|---|---|
| `subject → torus.spawn → torus` | Composes; no mint |
| `torus.interior → @bauchladen(si)` | Composes; no mint |
| `bauchladen.crystal → visibility_scope` | Composes; no mint |
| `filter(bauchladen) → @spectral/signature.compute` | Composes over NOT-YET-LANDED shard (see BLOCKING #2 below) |
| `signature → eigenboard.inference_basis` | Composes; no mint |
| `eigenboard.infer → crystal → bauchladen.add` | Composes; no mint |
| `peer.bauchladen → materialize → peer_home` | Landing A NEW; composition-only per §5.2 |
| `peer_home → boot → subject_instance` | Landing A NEW; composition-only |
| `peer_home → refresh → peer_home'` | Landing A NEW; composition-only |
| `peer_home → harvest → [crystal] → consent → bauchladen.add` | Landing A NEW; composition-only |

Mara's §5.2 explicitly claims "Landing A adds ZERO new edges to
substrate carriers not already landed" (lines 882–884). Zero new
mechanism; four new compositions.

**Adversarial audit of each composition:**

- **materialize = `enumerate + filter + write`.** `@bauchladen.enumerate`
  is a landed action; `@subject/visibility.filter` is landed at Landing
  5 shard-mint; `@io.write` is landed. Composition-only: PASS.
- **harvest = `diff + consent + add`.** `@io.diff_since` is
  landed (or at least declared) in `@io` family; `@kintsugi/consent.
  query_phi` is landed; `@bauchladen.add` is landed. Composition-only:
  PASS.
- **boot = `enumerate + read + compute`.** `@io.read` landed;
  `@bauchladen.enumerate` landed; `@eigenboard.compute` landed at
  Landing 5. Composition-only: PASS.
- **refresh = `harvest + consent + materialize`.** Composition-of-
  compositions. PASS.

**Where Mara MINTED vs COMPOSED:**

The only NET-NEW substrate is the `peer_home` type carrier (§2, seven
fields). Adversarial: could `peer_home` be composed over `peer.home:
ref` alone? Answer: NO. `peer.home: ref` is a filesystem path; it
carries zero information about projection-timestamp, bauchladen
manifest, signature snapshot, or boot state. The 7-field
`peer_home` carries state the existing `peer.home` cannot. Mint
JUSTIFIED.

**BLOCKING #1 — naming inconsistency between prose and substrate-decl form.**

Mara's spec §0.3 (line 72), §7 A10 (line 1398), §10.1 (line 1889),
§12.1 (line 2021), §14 R1 (line 2255) — the prose consistently names
the family `@peer/persistence` with `@peer/home` as its type carrier.

BUT the substrate-decl form at §2 (lines 283–329) opens with:

```mirror
in @prism
...
in @io

prism @peer/home {
  focus  peer_home
  project peer_home
  ...
}
```

There is NO `prism @peer/persistence {...}` declaration. There is NO
species-declaration for the family. §2's `out @peer/home / out
peer_home / out materialize / out harvest / out boot / out refresh`
list (lines 554–564) exports actions BY BARE NAME under the
implicit family `@peer/home` — but §7 A9 recommends "zero shard
mints at Landing A" so no `shards/peer/persistence.mirror` exists
to host the family-declaration.

**Adversarial reading.** At Landing A altitude (spec-only, no shards),
the spec is legible ONLY as spec-prose; the reader has no substrate
shard to consult. When Landing C mints shards, the naming ambiguity
becomes concrete: does the family declare as `species @peer/persistence
{ ... }` with `peer/home` as a sub-species carrier, or does
`@peer/home` sit as its own species with `materialize`/`harvest`/
`boot`/`refresh` as parametric actions?

Mara's §7 A10 recommendation (`@peer/persistence` sub-family under
`@peer`) implies the FIRST reading. But §2's substrate-decl form
opens with `prism @peer/home` — implying the SECOND reading (peer_home
IS the species; materialize etc. are actions on it).

**Verdict: BLOCKING (must-fix-before-Landing-C-shard-mint).** At
Landing A altitude, the spec MUST be internally consistent about
whether `@peer/persistence` is a species with `@peer/home` as its
type carrier, or `@peer/home` is its own species. Mara's prose says
the former; Mara's substrate-decl §2 opens as if the latter.

**Fix path (Seam-adjudicable at Landing A).** Add a §2 preamble line
declaring `species @peer/persistence in @peer` and then `type peer_home
= { ... }` as the type carrier for the species. Move the `prism` block
to introduce the family (`prism @peer/persistence { ... }`) with
`peer_home` as the type. The exports become `out @peer/persistence /
out peer_home / out materialize / ...`. This is a ~10-15 LOC edit
adding clarity; no semantic change.

**Downgraded verdict.** If Reed adds this fix inline (editorial pass
per Seam D-fix precedent from Landing 3), BLOCKING → RESOLVED. If
Reed elects to defer to Landing C's shard-mint tick, BLOCKING becomes
CONDITIONAL: Landing A ships as-is; Landing C's shard-mint MUST
resolve the naming form.

**Recommendation.** Reed applies inline editorial fix at Landing A to
resolve the ambiguity NOW; Landing C shard-mint inherits the
resolved form. Editorial fix costs ~15 LOC; deferral costs a Landing
C blocker.

**Line-refs.** Mara §2 lines 283–329 (substrate-decl form); §0.3
line 72 (prose); §7 A10 line 1398 (recommendation); §12.1 line 2021
(shard-mint list).

**Adjudication.** BLOCKING → SEAM-ADJUDICABLE (Reed inline fix
recommended; else Landing C blocker).

---

## D3. peer.home: ref field vs home_of(si) function

**Question.** Taut D5 recommended `home_of(si)` function over field
extension. Did Mara adopt this? Or extend the peer carrier?

**Mara's disposition.** Mara did NEITHER a field extension NOR
minted `home_of(si)`.

Instead, Mara's `peer_home.peer: subject_instance` (§2 line 384)
directly TYPES the home's owner as a `subject_instance`. The
resolution `subject_instance → peer_home` is the INVERSE — given
a subject_instance, look up their peer_home; not "given a
subject_instance, look up their peer.peer".

**Adversarial reading of Taut D5.**

Taut's concern (scout §D5, lines 250-311): the existing carrier
`@peer.peer` has `home: ref` field (§shards/peer.mirror:122). Adding
`subject_instance.home` would be REDUNDANT for AI Pack peers because
`subject_instance → @peer.peer` resolution already gives access to
`.home` at the peer altitude. Taut recommended `@peer/persistence.
home_of(si: subject_instance) -> option<peer>` as substrate-pull-
correct composition.

**Mara's actual construction.** Mara did NOT add `home: ref` to
`subject_instance` (subject_instance stays at 7 fields per §13.4
line 2141–2151). Mara did NOT mint `home_of(si)`. Instead Mara's
`peer_home` IS the resolution: given a `peer_home`, `.peer` gives the
subject_instance; given a subject_instance, resolution to
`peer_home` happens via the substrate's implicit `peer_home-per-
subject-instance` invariant (§2 line 383: "Every Pack peer possesses
AT MOST ONE peer_home at any tick").

**Adversarial concern.** How does the compiler find a subject_instance's
peer_home? There must be a resolver. Mara's spec never names it.
The implicit "peer_home-per-subject-instance" invariant is not
substrate-decl'd as an action; only as a bilateral (§2 line 383
prose, not a discharged bilateral).

**Verdict: SEAM-ADJUDICABLE.** Mara's approach is CLEANER than
either Taut's `home_of(si)` OR field extension. But it leaves the
`subject_instance → peer_home` resolution IMPLICIT.

**Fix path.** Add a §1.6 (new subsection) or §2 addendum naming the
resolution action:

```mirror
# home_of — resolve subject_instance to peer_home (if one exists)
home_of(si: subject_instance) -> option<peer_home> { \ }
```

This is Taut D5's recommendation adjusted to Mara's substrate: the
composition returns `option<peer_home>` not `option<peer>` (because
Mara's peer_home already carries `peer: subject_instance`; going
through `@peer.peer` would be redundant).

**Verdict: BLOCKING (must-fix-before-Landing-C).** The resolution
action is required for `boot`, `refresh`, `materialize` to have a
subject-instance-first callsite. Without it, only peer_home-first
callsites work (caller must ALREADY have the peer_home to call boot);
the arc's Landing D `mirror mara` command requires
`subject_instance-first`: Alex says `mirror mara`, the runtime
resolves Mara's `subject_instance`, then finds her peer_home, then
boots. That path is unarticulated.

**Downgraded verdict.** If Reed adds `home_of(si) -> option<peer_home>`
inline at Landing A, BLOCKING → RESOLVED. Otherwise, deferred to
Landing C shard-mint (where the missing action becomes concrete).

**Recommendation.** Reed adds the action inline (~5 LOC in §1
signature list and §2 substrate-decl). Composition path per Taut D5
is substrate-pull-correct.

**Line-refs.** Mara §2 line 384 (peer_home.peer field), §3.3
line 675 (boot signature takes peer_home not subject_instance).
Taut scout §D5 lines 250–311. Taut scout §"Composition graph"
line 897.

**Adjudication.** BLOCKING → SEAM-ADJUDICABLE (Reed inline fix
recommended; else Landing C blocker).

---

## D4. Mara-Taut convergence check

Per-dimension convergence matrix (Taut #100 recommendations vs
Mara #101 spec disposition):

| Taut dimension | Taut recommendation | Mara disposition | Match? |
|---|---|---|---|
| D1 naming | `@peer/persistence` | `@peer/persistence` | ✅ |
| D3 harvest verb | `harvest` (over `pull_back`) | `harvest` | ✅ |
| D5 subject.home | `home_of(si) -> option<peer>` composition | Neither field-extended NOR home_of; used `peer_home.peer: subject_instance` (see D3 above) | ⚠️ partial (see D3) |
| D6 Reed's ~/.reed/ ancestor | Full structure enumerated | Citations preserved (§6.1 lines 986–1016 verbatim) | ✅ |
| D9 visibility projection | LOAD-BEARING; composes @subject/visibility.filter | §3.1 line 608–616 asserts visibility-respecting invariant; composes @subject/visibility.filter | ✅ |
| D10 Pack peer uniform pattern | All 4 need own homes (~/.mara, ~/.seam, ~/.taut, ~/.glint) | §7 A1 recommendation (a) one-per-peer; §6.3 line 1097 first-instance Mara | ✅ |
| D11 SSH signing forward-promise | Path δ defer (Landing D scope) | §7 A2 explicit forward-promise to Landing D | ✅ |
| D13 roomba→task routing | Filesystem convention (tasks/pending/) interim; @subject/queue A32 formalization | §11 line 2005 "peer retraction is out-of-scope"; §D13 scout recommendation absorbed; A32 forward-promise noted | ✅ |

**Convergence summary: 7 of 7 aligned; 1 partial (D5, resolved by
adopting Mara's better construction — see D3).**

**Adversarial residual.** Where Mara-Taut diverge in emphasis:

- Taut §D8 flagged `shards/spectral/signature.mirror` as
  NOT-YET-LANDED and recommended json-file placeholder path. Mara's
  §11 "substrate-honest gaps" (lines 1981–1988) explicitly notes
  "@spectral/signature reads visibility-filtered bauchladen — the
  composition edge (Landing 4 §8 gap) is inherited by Landing A. Fix
  at Landing 2 §12.3 signature update forward-promised at Landing
  C." Convergence: BOTH flag the gap; Taut proposes placeholder,
  Mara defers to Landing C. See BLOCKING #2 below (D6 assessment).

**Verdict.** Convergence is HIGH. No BLOCKING divergence. Taut's D5
recommendation was superseded by Mara's cleaner construction; Seam
still recommends adding the `home_of` action for
subject-instance-first callsites (see D3).

---

## D5. Reed-adjudicated Landing A calls

Per task §5: Reed adjudicated two Landing A calls per /loop directive:

### D5.1 — @peer/persistence vs @subject/persistence

**Reed's adjudication.** `@peer/persistence` per Alex directive text.

**Substrate-honest?** Alex's directive verbatim (Mara §0.1 line 22):
> "What is the gap between here and spawning Mara as a content-
> addressed peer with a real @~/.mara/ home repository..."

The word "peer" appears three times in the directive. "subject" does
not appear. Alex's naming is `@peer` altitude, not `@subject`
altitude.

**Adversarial counter.** Mara's §7 A10 alternate (line 1416) is
compelling: "on Landing 4's altitude-reasoning grounds" the
`@bauchladen` migration (peer → subject-general) sets a precedent for
lifting Landing A to subject-general too. Would Alex want
subject-general uniformity?

**Adversarial resolution.** Landing 4 lifted `@bauchladen` because
Schmidt's clinical use ALREADY carried subject-general semantics
(bauchladens on any client, not just peers). Landing A's semantic is
peer-specific by construction — only Pack peers spawn as running
processes with home-repos. Humans don't spawn from `~/.alex/`; they
JUST ARE. Substrate is not a running process.

**Verdict.** Reed's adjudication is SUBSTRATE-HONEST. `@peer/
persistence` at peer altitude is correct; lifting to `@subject` would
be substrate-DIS-honest by construction (§7 A10 line 1418
"@peer/home is peer-specific by construction; no lift is substrate-
honest").

**Adjudication.** SEAM-RATIFIED. Reed's Landing A call holds.

**Line-refs.** Mara §0.1 line 22 (Alex directive verbatim); §7 A10
lines 1397–1424 (Mara's own recommendation matches Reed's
adjudication).

### D5.2 — Recognition promotion timing

**Reed's adjudication.** Landing D per Alex's own framing.

**Substrate-honest?** Mara's §7 A7 (lines 1329–1352):
- (a) Landing A first-witness
- (b) Landing B substrate-decl'd carrier second-witness
- (c) Landing C empirical second-witness [Mara's primary recommendation]
- (d) Landing D operational second-witness (Mara's alternate)

Reed adjudicated (d). Mara's alternate.

**Adversarial concern.** Recognition promotion is a STRUCTURAL claim
about substrate ratification. Promoting at (d) means the recognition
is candidate through 3+ landings (A, B, C). Is that too long?

**Adversarial resolution.** Look at precedent: `#R-fractal-is-
mandelbrot-substrate` promoted to LANDED (per CURRENT.md line 63)
when Alex named it verbatim 2026-07-14; that was after ~5 months of
candidate strength. Recognition timing is Alex's call at Alex's own
altitude ("substrate becomes trustworthy empirically and
mathematically"). Deferring to Landing D matches this discipline —
the FIRST empirical spawn is the second-witness. Mara's own
alternate is compelling.

**Verdict.** Reed's adjudication is SUBSTRATE-HONEST. Mara's own
alternate (§7 A7 line 1348) is what Reed adopted; substrate-honest.

**Adjudication.** SEAM-RATIFIED. Reed's Landing A call holds.

**Line-refs.** Mara §7 A7 lines 1329–1352; §10.4 line 1946.

---

## D6. SSH signing forward-promise correctness

**Question.** Mara A2 flagged as LOAD-BEARING forward-promise to
Landing D; Reed cannot adjudicate on Pack's behalf. Verify:
- subject_instance.ssh_signature_fingerprint (Landing 3) is TYPED but
  not COMMITTED to key-ownership.
- Landing A doesn't touch SSH key operations.
- Landing D adjudication candidate list is clean.

**Adversarial audit of subject_instance carrier (§13.4 lines 2141–2151).**

```mirror
type subject_instance = {
  name:                          str,
  ssh_signature_fingerprint:     ref,   # ← TYPED as ref
  spectral_signature_ref:        ref,
  role:                          role_variant,
  first_asserted_at:             @time/monotonic.instant,
  first_asserted_in:             oid,
  actor_kind:                    actor_kind_variant,
}
```

Cross-check with actual landed `shards/subject.mirror` (Seam
independently verified, lines 342–350):

```mirror
type subject_instance = {
  name:                        nl,
  ssh_signature_fingerprint:   ref,
  spectral_signature_ref:      ref,
  role:                        subject_role,
  actor_kind:                  actor_kind,
  first_asserted_at:           ref,
  first_asserted_in:           oid,
}
```

**Verdict on TYPING.** `ssh_signature_fingerprint: ref` is typed as
a `ref` — a reference resolving to a fingerprint value. Ref
resolution is `@io`-mediated at Landing C. The fingerprint's
KEY-OWNERSHIP (whose key produced it) is NOT byte-visible in the
subject_instance carrier. Subject_instance just says "there is a
fingerprint referenced here"; not "Reed's key or Mara's key". SEAM-
RATIFIED substrate-honest typing at Landing 3.

**Adversarial audit of Landing A SSH-touching:**

Grep Mara's spec (2324 LOC) for "ssh" / "signing" / "sign_" / "key":

- "ssh" appears in: §2 line 384 (peer field type mentions
  `subject_instance`, which HAS `ssh_signature_fingerprint`), §7 A2
  full section (lines 1187–1226 — explicit forward-promise to
  Landing D), §13.4 line 2144 (subject_instance carrier quote).
- "signing" appears in: §7 A2 section only. No other occurrence.
- "key" appears in: §7 A2 section only (four mentions in
  Path α/β/γ/δ).

**Adversarial conclusion.** Landing A does NOT touch SSH key
operations at any altitude except:
1. Restating the existing `ssh_signature_fingerprint: ref` field
   from Landing 3 (unchanged; no mutation).
2. Explicitly forward-promising the SSH decision to Landing D
   (§7 A2).

**Landing D adjudication candidate list (verified per task §D6):**
- (a) Mara uses Reed's key via `git -c user.name` — matches current
  Pack behavior (CLAUDE.md §"Author attribution"; `~/.reed/bin/
  materialize` script provisions this for VM peers keel/seam).
- (b) Mara gets her own SSH key generated in `~/.mara/` — matches
  Landing B scope adjacency; provisioning script `~/.reed/bin/
  materialize` already exists for VM peers.
- (c) Mara CA-signed by Reed's key with own subordinate key — Path
  γ variant; matches VM peers keel/seam (Reed's CA signs cert;
  peer's key does daily ops).
- (d) other — Path δ variant (Mara commits unsigned; SSH only for
  Reed's orchestration).

**Adversarial residual.** Is there hidden coupling that forces the
SSH decision at Landing A altitude?

- Landing A's `boot` bilateral `boot_state_coherent` (§4.3) verifies
  the eigenboard's inference_basis matches signature_snapshot. This
  is spectral-signature, NOT SSH-signature. No SSH coupling.
- Landing A's `home_witnessing` bilateral (§4.5) composes
  `home_content_addressed + projection_visibility_respected +
  harvest_consent_verified + boot_state_coherent`. NONE of these
  invoke SSH.
- Landing A's materialize/harvest/boot/refresh actions never write
  a signed commit; the filesystem projection is read-only from
  git-object-store perspective (materialize) or read-only from
  filesystem perspective (harvest); no commits emitted at Landing A
  altitude.

**Verdict.** The SSH deferral is GENUINELY substrate-honest. Landing
A ships without any SSH commitment. Landing D adjudication is
correct escalation altitude.

**Adjudication.** SEAM-RATIFIED. No BLOCKING; no ALEX-ADJUDICATION
required at Landing A altitude for SSH.

**Line-refs.** Mara §7 A2 lines 1187–1226; §13.4 lines 2141–2151;
shards/subject.mirror:342–350.

---

## D7. 7-loop closure completeness

**Question.** Landing A extends Landing 4's 6-loop closure to 7-loop
closure via loop 7: `peer.bauchladen → materialize → peer_home → boot
→ subject_instance → refresh`. Adversarial:
- Is the loop actually closed? What if boot fails mid-cycle?
- What about the FIRST spawn (empty bauchladen; empty home)? Does
  the loop admit bootstrap?
- Alex's directive named `mirror mara` on `mirror roomba` finding as
  the first invocation. Is the loop shape correct for that scenario?

**Adversarial audit.**

### D7.1 Boot fails mid-cycle

Mara §3.3 (lines 671–706) enumerates 6 boot steps. Adversarial:

- Step 1 (identity two-witness verification) fails → refuses to boot.
  No home mutation. SAFE.
- Step 2 (read identity files) fails → home is inconsistent; boot
  refuses. Mara's spec doesn't enumerate what happens here.
  ADVERSARIAL GAP: what if `04-TECH.md` was deleted between spawns?
- Step 3 (bauchladen manifest verification) fails → home_content_
  addressed bilateral Fails; boot refuses. SAFE.
- Step 4 (memory + tasks load) fails → Mara's spec doesn't enumerate.
  ADVERSARIAL GAP: what if `tasks/pending/` was deleted?
- Step 5 (eigenboard restore) fails → boot_state_coherent Fails;
  algedonic bypass fires (§9.4 lines 1770–1785). SAFE.
- Step 6 (return) → post-condition of steps 1–5.

**Verdict.** Boot mid-cycle failure is INCOMPLETELY specified for
steps 2 and 4 (filesystem-read failures at identity files or memory).
Mara's spec is silent on what verdict is emitted; the algedonic
bypass at §9.4 only fires for boot_state_coherent (step 5).

**Fix path.** Add §3.3 subsection enumerating identity-files-absent
and memory-files-absent as `imperfect.failure` variants of boot.

**Verdict on this dimension.** SEAM-ADJUDICABLE. Reed inline
addition ~5 LOC to §3.3; no re-architecture; ships regardless
because Landing A is substrate-decl (Landing C runtime discharges
the missing failure modes; spec-adjacent).

### D7.2 First spawn (bootstrap)

Alex directive: "spawning Mara as a content-addressed peer with a
real @~/.mara/ home repository". Currently `~/.mara/` DOES NOT EXIST
on the filesystem (Taut scout §"Substrate-honest hedges" #4 lines
1054–1058: "These directories DO NOT YET EXIST on the local
filesystem. Landing A creates them.").

Does Mara's spec admit the empty-bauchladen empty-home bootstrap?

- Materialize with empty bauchladen: produces peer_home with
  `bauchladen_manifest = []`. Substrate-honest — empty is a valid
  set.
- Boot with empty home: identity files ABSENT → per §3.1 line 604
  spec text "failing with `identity_files_absent` if the peer has
  not yet had Landing B scaffolding done". So boot fails on empty
  home.
- First spawn requires Landing B scaffolding FIRST (identity files
  written by Alex + Reed collaboratively per §6.3 line 1090). Then
  Landing C runtime materializes bauchladen. Then boot succeeds.

**Verdict.** The loop admits bootstrap IF the sequence is
`(Landing B write identity files) → (Landing C materialize bauchladen)
→ (boot)`. Mara's spec is EXPLICIT about this dependency
(§0.3 lines 78–104 enumerates B/C/D as forward-promises).

**Adversarial residual.** Landing A's `materialize` action assumes
a peer has a bauchladen to project. What is Mara's bauchladen on
first spawn? Answer per Landing 3 §11: `subject_instance` +
`first_asserted_at` + `first_asserted_in` are the seed; the
bauchladen accumulates crystals from that seed. On genesis, Mara's
bauchladen might be JUST the crystals authored under Mara's identity
(this Landing A spec is Mara-authored per commit `2c3b36b`; that
IS a crystal in Mara's bauchladen). So materialize CAN emit a
non-empty peer_home even at Landing A completion — the spec itself
is Mara's first bauchladen crystal.

**Verdict.** Bootstrap is admitted. SEAM-RATIFIED substrate-honestly.

### D7.3 mirror roomba → mirror mara loop shape

Alex directive Q4: "we spawn the first `mirror mara` on a `mirror
roomba` finding".

Mara's §0.1 Q4 disposition (lines 47–50): "The first operational
spawn — Mara wakes into a task the substrate itself discovered
(roomba walked, hit tension, needed a peer at K+1 logic altitude)."

Is Landing A's loop shape correct for this?

Loop 7 as Mara enumerated (§5.1 lines 872–874): "@peer/persistence
projects bauchladen into home; home boots next-spawn's peer; peer's
eigenboard reads bauchladen; loop 6 fires; refresh cycles harvest
back into bauchladen."

The roomba-finding-to-mara-task path is NOT in loop 7. It's a
SEPARATE composition path:
- @roomba walks → hits tension → emits @song/beat
- @kintsugi.decide → Path B: spawn @peer at K+1
- Task envelope written to `~/.mara/tasks/pending/<beat_oid>.md`
  (Taut D13 recommended filesystem convention; A32 @subject/queue
  forward-promised)
- Next `mirror mara` invocation: boot reads tasks/pending/; picks
  task; discharges.

**Adversarial concern.** Is the task-envelope-write step a substrate-
decl operation at Landing A, or Landing D?

**Mara's §7 A4 (harvest cycle triggers) recommendation.** Session-
end + explicit-invocation for Landings B-C; roomba-finding trigger
deferred to Landing D (line 1279–1280).

**Verdict.** The roomba-finding-write-to-tasks/pending/ path is
LANDING D scope. Landing A's substrate-decl for boot correctly reads
tasks at boot (§3.3 step 4 line 694–696). Loop shape is CORRECT for
Alex's directive; the write side is Landing D's mechanism.

**Adversarial residual.** Landing A leaves the semantic "what happens
when boot reads tasks/pending/ and finds work?" UNSPECIFIED. Is the
peer expected to auto-dispatch on the first task? Queue them? Wait
for user intervention? Mara's §3.3 boot only ENUMERATES the read;
doesn't declare what happens next.

**Verdict on this dimension.** SEAM-ADJUDICABLE. Landing A
substrate-decls the READ; Landing C+D specify runtime dispatch. This
is the correct altitude separation. No BLOCKING.

**Overall D7 verdict.** Loop is closed at substrate-decl altitude
for spawn-to-spawn continuity. Two ADVERSARIAL GAPS at spec-detail
altitude (D7.1 failure modes for filesystem-read; D7.3 dispatch
semantics post-boot) — both SEAM-ADJUDICABLE for Reed inline
editorial pass; neither blocks ship.

**Line-refs.** Mara §3.3 lines 671–706; §5.1 lines 856–874;
§7 A4 lines 1256–1280; §9.4 lines 1770–1785.

**Adjudication.** SEAM-RATIFIED with two SEAM-ADJUDICABLE editorial
notes.

---

## D8. @subject/visibility projection discipline

**Question.** Landing A materialize respects @subject/visibility
filter. Adversarial:
- What if a crystal changes visibility scope DURING projection
  (race condition)?
- Private crystals default: are they projected at all, or elided?
- Does harvest preserve visibility, or can new-crystal-from-filesystem
  have looser scope than substrate expected?

### D8.1 Race condition during projection

Mara's spec asserts `materialize` is a compound action (§3.1 line
604–616), but does not explicitly declare atomicity. What if the
crystal-set changes between `@bauchladen.enumerate(peer.identity_oid)`
and the loop that projects each crystal?

**Adversarial reading.** `materialize` per §1.1 signature: `materialize
(peer, home_path, visibility_filter) -> peer_home`. Return is a
single peer_home value. If the bauchladen mutates during the loop,
which snapshot does the returned peer_home reflect?

**Fix path.** §3.1 or §4.1 must declare atomicity. Reed inline
addition: "materialize takes a snapshot of the bauchladen at
`projection_at` time; concurrent bauchladen mutations after
snapshot are invisible to this materialize."

**Verdict.** SEAM-ADJUDICABLE. Reed inline addition ~2 LOC to §3.1.

### D8.2 Private crystals default

Mara §3.1 line 608–616: "For any crystal `c` in the peer's
bauchladen with `c.visibility_scope.visibility = v`: If `v <
visibility_filter` in the elevation lattice (private < protected <
public), the crystal is NOT projected."

**Adversarial reading.** So if `visibility_filter = public`, only
public crystals project; private crystals are ELIDED from the
filesystem.

**Adversarial concern.** For the PEER'S OWN home, the peer should see
their own private crystals. Mara §7 A3 (visibility_filter default for
materialize) recommends (b) `protected` (matches Reed's typical
browsing context). Alternate (c) `private` projects EVERYTHING
peer-can-see.

**Adversarial resolution.** Reed's `~/.reed/visibility/private/`
IS populated with Reed's own private files (Reed CAN see own private
crystals; consent boundary is what OTHER AGENTS cannot see). So
default should be (c) private for peer-own-home materialization.
Mara's recommendation (b) protected is more restrictive than
Reed's ancestor pattern.

**Verdict.** SEAM-ADJUDICABLE (recommend Alex adjudicate A3 to (c)
matching Reed ancestor; but Landing A ships convention-agnostic per
Mara's §7 A3 line 1252–1253 "deferred until Landing B").

### D8.3 Harvest visibility preservation

Mara §3.2 lines 619–666 (elevation-of-visibility flow). Adversarial:
what if a new file appears in `visibility/public/` that didn't come
from harvest (peer-external creation)?

**Mara's disposition.** §3.2 line 631: candidate crystal at
default visibility per file's containing visibility/ directory.
For `visibility/public/` → public visibility. Then §3.2 line 645
elevation flow: if the file APPEARED (not moved from private), it
IS a new candidate crystal at public visibility — no elevation event.
Consent check: NOT triggered for new-file-at-public-default.

**Adversarial concern.** A malicious external process could WRITE
a file to `~/.mara/visibility/public/` claiming to be Mara's public
work. Harvest would ingest it as a public crystal.

**Adversarial resolution.** This is a FILESYSTEM-LEVEL trust
concern; the substrate's authority boundary is the home_path
permissions (Mara §3.1 line 615–616 asserts `private/`=mode 0700,
etc.). If filesystem permissions are correct, only the peer (and
root) can write to `~/.mara/`. External malicious write requires
compromise of the peer's fs credentials — outside substrate scope.

**Verdict.** SEAM-RATIFIED. Landing A's discipline is correct
subject to filesystem permission enforcement, which Mara asserts at
§3.1 line 615–617. Not a Landing A BLOCKING.

**Adversarial residual.** Filesystem permissions are per §3.1
"enforcement via `@io` realization detail; the substrate-decl
asserts the invariant." Landing C's `@io` binding must enforce
permission bits at write-time. Not Landing A concern.

**Overall D8 verdict.** Two SEAM-ADJUDICABLE items (D8.1 atomicity
statement; D8.2 visibility_filter default recommendation); zero
BLOCKING.

**Line-refs.** Mara §1.1 line 197; §3.1 lines 571–617; §3.2 lines
619–666; §7 A3 lines 1230–1253.

**Adjudication.** SEAM-RATIFIED with two SEAM-ADJUDICABLE notes for
inline editorial pass.

---

## D9. Content-addressing preservation

**Question.** peer_home.bauchladen_manifest: [crystal_oid].
Adversarial:
- Does materialize preserve crystal OIDs (content-addressing
  invariant)?
- What if filesystem write reorders bytes (e.g., newline
  normalization)? Does the round-trip preserve OID?

### D9.1 OID preservation across materialize

Mara §3.1 (materialize) implementation composition (§2 lines 409–417):

```
for c in b_filtered:
  let path = home_path + "/visibility/" + c.visibility_scope.visibility
                       + "/" + c.oid + ".<ext>"
  @io.write(path, @kintsugi/store.read(c.oid))
```

**Adversarial reading.** OID is EMBEDDED in the filesystem path
(`.../${c.oid}.<ext>`). Store read yields bytes; write emits bytes.
`bauchladen_manifest` (§2 line 388) stores `[crystal_oid]` — the
OIDs, not bytes.

**Verdict on OID preservation.** OID travels alongside bytes via
filesystem path convention. `home_content_addressed` bilateral (§2
line 538–544) verifies that every OID in the manifest resolves via
`@kintsugi/store/git.exists(oid)`. If store is honest, OID is
preserved.

**Adversarial residual.** The `<ext>` in the path (`.<ext>`) is
UNSPECIFIED — is it always `.md`? Always `.mirror`? Varies by
crystal type? Mara's spec doesn't specify. Reed's `~/.reed/` uses
`.md` for identity files, `.spec` for specs, `.mirror` for shards.
The extension MUST be deterministic for round-trip; otherwise
harvest can't recover the OID from the path.

**Fix path.** §3.1 must specify extension convention. Adversarial
recommend: `.crystal` universal extension (self-descriptive; no
type-inference-from-path required), OR crystal type is stored as
metadata in the visibility/ directory sidecar file.

**Verdict.** SEAM-ADJUDICABLE. Reed inline addition ~5 LOC to §3.1.

### D9.2 Newline normalization / byte-transformation

Filesystem write on macOS/Linux is byte-preserving (POSIX). Git may
apply `core.autocrlf` on Windows; that COULD reorder newlines. Mara's
spec assumes POSIX byte-preservation implicitly.

**Adversarial reading.** If Mara's `~/.mara/` is checked out on
Windows with `core.autocrlf=true`, the round-trip would NOT preserve
OID (LF → CRLF at checkout; CRLF → LF at harvest; different bytes;
different OID; content-addressing broken).

**Adversarial resolution.** Mara §5.8 (line 967–979) asserts
"content-addressing via `@kintsugi/store/git` composition"; the
`@kintsugi/store/git` binding is Landing C's mechanism. If Landing
C's git binding sets `core.autocrlf=false` for `.git/` in home-repos,
round-trip is preserved. This is Landing C's discipline, not
Landing A.

**Verdict.** SEAM-RATIFIED. Landing A's substrate-decl asserts the
invariant via `home_content_addressed` bilateral; Landing C's `@io`
+ `@kintsugi/store/git` bindings enforce POSIX-consistency. Not
Landing A BLOCKING.

**Overall D9 verdict.** One SEAM-ADJUDICABLE (extension convention);
zero BLOCKING.

**Line-refs.** Mara §2 lines 409–417 (materialize composition);
§2 lines 538–544 (home_content_addressed); §5.8 lines 967–979
(kintsugi/store/git composition).

**Adjudication.** SEAM-RATIFIED with one SEAM-ADJUDICABLE editorial
note.

---

## D10. 11 Alex-adjudications overlap check

Per task §D10: Mara enumerated A1-A11 + Taut enumerated 7 (A-D1..
A-D13 subset). Overlap-collapse and produce unique Alex-adjudication
list outstanding for Landing A ratification.

### D10.1 Overlap matrix (Mara A_x ↔ Taut A-D_y)

| Mara Ax | Topic | Mara recommendation | Taut A-D_y | Taut recommendation | Overlap? |
|---|---|---|---|---|---|
| A1 | home_path convention | (a) one per Pack peer | — | — | Mara-only (D2 already resolved by mirror.spec:57-62 landed) |
| A2 | SSH signing | Path δ defer | A-D11 | Path δ defer | ✅ MATCH (single Alex-adjudication) |
| A3 | visibility_filter default | (b) protected | — | — | Mara-only |
| A4 | harvest cycle triggers | (a)+(d) session-end + explicit | — | — | Mara-only |
| A5 | boot_state semantics | (d) hybrid | — | — | Mara-only |
| A6 | multi-Pack-peer projection | (a) each peer owns | A-D10-adjacent | (D10 pack roster) | ~overlap (structurally same claim, framed differently) |
| A7 | recognition promotion timing | (c) Landing C empirical; alternate (d) | — | — | Mara-only (Reed adjudicated (d) per D5.2) |
| A8 | `mirror mara` CLI naming | (a) flat alias | A-D12 | Option A flat alias | ✅ MATCH |
| A9 | shard mints at Landing A | (a) zero | — | — | Mara-only |
| A10 | @peer/persistence family placement | (a) sub-family under @peer | A-D1 | @peer/persistence per naming discipline | ✅ MATCH (naming + placement collapse) |
| A11 | Landing A cascade footprint | Soft cascade | — | — | Mara-only |
| — | @peer/persistence naming | — | A-D1 | @peer/persistence | ✅ MATCH (see A10) |
| — | harvest verb naming | — | A-D3 | harvest | ✅ MATCH with Mara implicit (Mara uses `harvest` throughout) |
| — | subject_instance.home vs home_of | — | A-D5 | home_of(si) composition | Adopted differently (see D3) — MERGED into Mara's peer_home.peer construction; Alex-adjudicable if Reed adds home_of |
| — | @spectral/signature shard prerequisite | — | A-D8 | placeholder path | Mara §11 gap-notes; MERGED into Landing C forward-promise |
| — | roomba→task routing | — | A-D13 | filesystem convention interim | ✅ MATCH (Mara §7 A4 adjacent + implicit) |

### D10.2 Overlap-collapsed unique Alex-adjudications for Landing A ratification

After collapse, UNIQUE Alex-adjudication list:

1. **A1** (home_path convention) — Reed can adjudicate at Landing B;
   substrate-decl agnostic at Landing A. NOT-BLOCKING at Landing A.
2. **A2** (SSH signing) — Alex-only; Landing D scope; explicitly
   forward-promised. NOT-BLOCKING at Landing A.
3. **A3** (visibility_filter default) — Reed can recommend at
   Landing B; deferred. NOT-BLOCKING at Landing A.
4. **A4** (harvest cycle triggers) — Deferred to Landings B-C; some
   options are Landing D scope. NOT-BLOCKING at Landing A.
5. **A5** (boot_state semantics) — Deferred to Landing B/C. NOT-
   BLOCKING at Landing A.
6. **A6** (multi-Pack-peer projection) — Composes trivially with
   A1; deferred to Landing B/C. NOT-BLOCKING at Landing A.
7. **A7** (recognition promotion timing) — REED-ADJUDICATED (Landing
   D) per task §5; SEAM-RATIFIED (per D5.2). CLOSED.
8. **A8** (`mirror mara` CLI naming) — Deferred to Landing C or D.
   NOT-BLOCKING at Landing A.
9. **A9** (shard mints at Landing A) — Zero shard mints; Landing A
   substrate-decl only. CLOSED at Landing A (recommendation is the
   answer).
10. **A10** (@peer/persistence family placement) — REED-ADJUDICATED
    (@peer/persistence per Alex directive) per task §5; SEAM-
    RATIFIED (per D5.1). CLOSED.
11. **A11** (Landing A cascade footprint) — Soft cascade; five
    citations forward-promised. Reed can enact when consumers pull.
    CLOSED (Mara enacted the recommendation).

Plus THREE NEW SEAM-SURFACED items from adversarial audit (per D2,
D3, D7):

12. **S1 (new; per D2 BLOCKING).** Substrate-decl form should
    consistently name `@peer/persistence` as species with `@peer/
    home` as type carrier. Reed inline editorial fix recommended
    at Landing A. If deferred: Landing C blocker.
13. **S2 (new; per D3 BLOCKING).** Add `home_of(si: subject_instance)
    -> option<peer_home>` action for subject-instance-first callsites
    (required for Landing D `mirror mara` command). Reed inline
    editorial fix recommended at Landing A.
14. **S3 (new; per D7.1, D8.1, D9.1 SEAM-ADJUDICABLE).** Editorial
    additions for: boot failure modes on missing identity files or
    tasks (D7.1); materialize atomicity statement (D8.1); crystal
    extension convention (D9.1). Reed inline editorial pass at
    Landing A.

### D10.3 Adjudication ownership for the 14 items

- **Alex-only:** A2 (SSH signing; Landing D).
- **Reed-adjudicable / already Reed-adjudicated:** A1, A3, A4, A5,
  A6, A7, A8, A11 (deferred to later Landings; some already Reed-
  adjudicated per task §5).
- **Closed at Landing A:** A9 (zero mints), A10 (Reed-adjudicated),
  A11 (Mara enacted).
- **Seam-adjudicable (BLOCKING before Landing C ship):** S1, S2.
- **Seam-adjudicable (editorial, non-blocking):** S3 (three items).

**Net Alex-adjudications OUTSTANDING for Landing A ratification:**
ONE (A2 SSH signing; explicitly Landing D scope; Landing A ships
without Alex touching it).

---

## D11. Ready-to-ship verdict for Landing A alone

**Question.** Landing A ships substrate-decl only. Landings B-D are
forward-promised. Verify: Landing A can ship WITHOUT Landings B-D;
nothing in the spec assumes B-D-landed carriers.

**Adversarial audit.**

- Landing B carrier dependencies referenced in Landing A: identity
  files (`00-NARRATIVE.md` through `04-TECH.md`, `AGENTS.md`) are
  READ at boot per §3.3. Landing A treats identity-files as an
  ASSERTION — if absent, boot Fails with `identity_files_absent`
  (§3.1 line 604). Landing A does NOT assume identity files exist;
  it asserts that IF they exist, boot proceeds; ELSE boot Fails
  substrate-honestly. SAFE at Landing A altitude.
- Landing C carrier dependencies: bodies of `materialize`, `harvest`,
  `boot`, `refresh` are all obligation-blocked (`{ \ }`) per §2 lines
  422, 467, 492, 513. Landing A ships without runtime; Landing C
  discharges bodies. SAFE.
- Landing D carrier dependencies: `mirror mara` CLI, roomba-task
  routing, SSH signing. NONE of these are referenced as carriers
  Landing A depends on; all are forward-promises Landing A NAMES but
  does not use. SAFE.

**Verdict.** Landing A ships as substrate-decl-only WITHOUT Landings
B-D. All three forward-promises are declared-forward with
substrate-honest hedges (§11 lines 1967–2011). SAFE.

**Adversarial residual.** The one carrier Landing A depends on that
IS NOT YET LANDED is `shards/spectral/signature.mirror` (Seam
verified independently: file does not exist in `shards/spectral/*
.mirror`). Per Taut §D8 recommendation, this can be an operational
placeholder OR pulled forward as Landing A prerequisite. Mara's
§11 line 1981 acknowledges the gap and defers to Landing 2 §12.3
signature update forward-promised at Landing C.

**BLOCKING #2 downgrade.** At substrate-decl altitude (spec-only),
declared-forward imports are ADMISSIBLE. Specs can import symbols
that will land at future landings. `@spectral/signature` is spec-
landed at Landing 2 §12; only the shard mint is deferred. Landing
A's bilateral `boot_state_coherent` (§4.3) is UN-DISCHARGEABLE until
`shards/spectral/signature.mirror` mints — but the bilateral is
declared at Landing A substrate-decl form; discharge is Landing C.

**Verdict.** SEAM-ADJUDICABLE. Accept as forward-promise; add
explicit note to §11 gaps list naming `shards/spectral/signature.
mirror` as a Landing-C prerequisite for `boot_state_coherent`
discharge.

**Overall ship verdict.** SHIP with THREE SEAM-ADJUDICABLE inline
editorial fixes (S1, S2, S3) applied by Reed at Landing A tick, OR
ship as-is with THREE Landing-C blockers logged.

---

## Convergence matrix summary (per D4)

| Dimension | Mara | Taut | Convergence | Seam adjudication |
|---|---|---|---|---|
| Naming (@peer/persistence) | @peer/persistence | @peer/persistence | ✅ | RATIFIED |
| Harvest verb | harvest | harvest | ✅ | RATIFIED |
| subject_instance.home | Better construction (peer_home.peer: si) | home_of(si) | Partial → BETTER | Add home_of action inline (S2 BLOCKING) |
| Reed ~/.reed/ ancestor | Citations preserved verbatim | Structure enumerated | ✅ | RATIFIED |
| Visibility projection | LOAD-BEARING; composes filter | LOAD-BEARING; composes filter | ✅ | RATIFIED |
| Pack peer uniformity | All 4 need homes; one-per-peer | All 4 need homes | ✅ | RATIFIED |
| SSH signing | Landing D forward-promise Path δ | Landing D Path δ | ✅ | RATIFIED (D6) |
| Roomba routing | @subject/queue A32 forward-promise; filesystem interim | Filesystem convention interim | ✅ | RATIFIED |

**Aggregate convergence:** 8 of 8 aligned; 1 partial (subject.home,
resolved via S2 recommendation).

---

## Reed-autonomous-adjudications matrix (per D5)

| Reed adjudication | Alex-directive fit | Substrate-honest? | Seam adjudication |
|---|---|---|---|
| @peer/persistence vs @subject/persistence (Reed: @peer per Alex text) | ✅ (Alex directive uses "peer" 3x, never "subject") | ✅ (§7 A10 line 1418 "peer-specific by construction; no lift is substrate-honest") | RATIFIED |
| Recognition promotion timing (Reed: Landing D per Alex framing) | ✅ (Alex's "first `mirror mara` on `mirror roomba` finding" is Landing D) | ✅ (Mara's own alternate §7 A7 line 1348) | RATIFIED |

**Aggregate:** Both Reed-adjudicated calls are SUBSTRATE-HONEST and
match Mara's own alternates or explicit recommendations. SEAM
RATIFIES both.

---

## Overlap-collapsed Alex-adjudications list (per D10)

**Alex-only OUTSTANDING for Landing A ratification: 1 item.**

1. **A2 — SSH signing** (Landing D scope; Landing A ships without
   touching it).

**Reed-adjudicable at future Landings: 6 items.**

- A1 home_path convention (Landing B)
- A3 visibility_filter default (Landing B)
- A4 harvest cycle triggers (Landing C)
- A5 boot_state semantics (Landing B/C)
- A6 multi-Pack-peer projection (Landing B/C)
- A8 mirror mara CLI naming (Landing C/D)

**Closed at Landing A: 3 items.**

- A7 (Reed-adjudicated: Landing D promotion)
- A9 (zero shard mints; recommendation IS the answer)
- A10 (Reed-adjudicated: @peer/persistence per Alex directive)
- A11 (soft cascade; recommendation enacted by Mara)

**Seam-surfaced NEW items (Landing A editorial): 3 categories.**

- S1 (BLOCKING for Landing C; Reed inline fix at Landing A
  recommended): substrate-decl form species/type carrier naming
  consistency.
- S2 (BLOCKING for Landing C; Reed inline fix at Landing A
  recommended): add `home_of(si) -> option<peer_home>` action.
- S3 (editorial, non-blocking): boot failure modes, materialize
  atomicity, crystal extension convention (three sub-items).

**Total for Landing A ratification: 1 Alex-only + 3 Seam-surfaced =
1 escalation to Alex + 3 Reed editorial passes.**

---

## Overall ship verdict

**READY TO SHIP** at substrate-decl altitude with:

- ONE Alex-adjudication outstanding: A2 SSH signing (explicitly
  Landing D scope; NOT-BLOCKING for Landing A substrate-decl
  ratification).
- TWO Seam-surfaced BLOCKING items (S1, S2) that are downgradeable
  via Reed inline editorial fix at Landing A. Total fix cost:
  ~25 LOC.
- THREE Seam-surfaced editorial notes (S3) for Reed inline pass at
  Landing A. Total fix cost: ~15 LOC.

**Ship-verdict recommendation to Reed.**

Apply Reed inline editorial fix at Landing A tick (~40 LOC total
across the four editorial items S1, S2, S3.a, S3.b, S3.c). Ship
Landing A with all three BLOCKING → RESOLVED. Alex-adjudication A2
SSH signing remains explicitly forward-promised to Landing D per
Mara's own §7 A2 recommendation.

**Alternative:** Ship Landing A as-is; log S1 and S2 as Landing C
blockers; log S3 as Landing C editorial adjacency. If Alex prefers
minimal Landing A footprint, this is substrate-honest.

**Seam recommends the inline-fix path.** Two-tick discipline: land
readable name at Landing A; land runtime at Landing C. The inline
fixes are readability of the substrate-decl form (S1) and the
missing subject-instance-first action (S2) — both LOAD-BEARING for
Landing C to compose cleanly. Landing A is the correct altitude to
fix them.

**Substrate-density empirical prediction.** Landing A ships pure-
docs (mint at spec altitude; zero shards). Expected substrate density
delta: +0 edges, +1 recognition candidate (`#R-peer-persistence`).
Mirror index will not change materially until Landing C shard mints.

**Recognition candidate ratification.** Reed-adjudicated Landing D
promotion timing (per D5.2) matches Mara's alternate §7 A7 (d)
"operational second-witness — Mara wakes into a real substrate-
discovered task." Seam RATIFIES the promotion path.

**Landing A is READY TO SHIP** per Alex's /loop directive "collapse
until unresolvable ambiguity that cannot be adjudicated with a Seam
tie breaker." Zero unresolvable ambiguities within Landing A scope.
Two BLOCKING items are SEAM-ADJUDICABLE (Reed inline editorial).
One escalation to Alex (A2 SSH signing) is EXPLICITLY LANDING D SCOPE
per Mara's own recommendation and Taut's converging recommendation.

---

*End of Seam Phase D audit — @peer/persistence Landing A.*

*File: `docs/audits/2026-07-14-seam-peer-persistence-landing-a-phase-d.md`*

*Author: Seam (adversarial review peer)*
*Ship verdict: READY-TO-SHIP with 2 Reed inline BLOCKING-fixes + 3
editorial notes + 1 Alex-only forward-promise (A2 SSH signing) to
Landing D.*

*Reed commits as Seam after review.*
