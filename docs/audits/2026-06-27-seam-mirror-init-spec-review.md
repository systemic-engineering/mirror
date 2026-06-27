## §1 — Position

Adversarial review of Mara's `mirror/docs/specs/mirror-init.md`
(commits `fe215bd` → `14dd043`, ~1208 lines after the §3.2 trim).
The spec lifts Taut's scout
`docs/scouts/2026-06-27-taut-fragmentation-git-store-for-mirror-init.md`
(`5580a7e`) from composition inventory into substrate-decl: declares
the declared-but-not-wired Cargo edge, names the three-deliverable
collapse, pins `.git/mirror/`, frames spawn↔recall↔init, addresses
Taut's R1+R2, and closes with §10's autopoietic recursion-lock.

Cascade: Taut scout → Mara spec → this review. Next gate: Reed RED
tick on the Cargo edge + the v0 indexer.

Read-only fences honored: no fragmentation/mirror code modified; no
substrate-decl introduced; no candidate promotion. Audits-genre
consolidation per `2026-06-26-seam-mirror-recall-spec-review.md`.

## §2 — Methodology

1. Read the spec end-to-end (1574 lines including frontmatter).
2. Read Taut's scout (400 lines) for the inventory baseline.
3. Verify every file-path + line-number in §3 against fragmentation
   source: `namespaced.rs:34`, `git.rs:124`, `frgmnt_store.rs:56`,
   `walk.rs:25`, `project.rs:42`, `crystallize.rs:510`.
4. Verify the "Cargo edge does not exist" claim with grep against
   `mirror/bootstrap/Cargo.toml` — confirmed (`fragmentation` appears
   ONLY in a `[[architecture-fragmentation-...]]` comment under
   `libc`'s docblock; no `[dependencies]` entry).
5. Verify cross-spec references: `mirror-store.md` §4.5/§4.6
   (Cuts 1+2 + Cargo line); `spawn-is-substrate-leaving-ground-
   state.md` §4.5 (idempotency claim); `spectral.engineer/garden/
   spectral-db/spec.md` §11 (autopoietic precedent).
6. Verify the envelope-shape claim in §4.7 against `cmd_spawn`'s
   actual JSON envelope at `bootstrap/src/lib.rs:3168`.
7. Per `[[feedback-substrate-already-had-the-word]]`: grep before
   asserting; every "exists" carries a citation; every "missing"
   carries a grep miss.

Severity legend: **C** critical (blocks the next tick) / **S**
substantive (architectural issue; not a blocker) / **L** light (style,
under-specified, polishable) / **✓** verified clean.

## §3 — Per-section seam list

### §0 Pre-position — [✓]
The pre-position is structurally distinct from §10: §0 ASSERTS the
recursion; §10 EARNS it. The form holds. The "latency bounded BELOW
by Cargo-edge tick + first-init-run" framing is honest about what
makes the autopoietic claim falsifiable. Good shape.

### §1 What `mirror init` IS — [✓]
The five verb-claims in §1.1 each map to a §4 step. §1.2's five
structural negatives are sharp; the "NOT a fork of `mirror spawn`"
claim correctly references the spawn insight without conflation. §1.3
names the architectural cut at the right altitude.

### §2 Declared-but-not-wired discovery — [L]
The two prior-instance citations (#89 `@mirror/ref` collision, #43
content-addressed build system) are accurate. The flag-not-promote
discipline (§2.2) honors the brief. **Light seam:** §2.3's claim
"substrate-decl drift outpaces wiring drift in the Pack's current
work cadence" is a sociological observation NOT yet evidenced — only
two instances cited; the 52+ instances of `[[feedback-substrate-
already-had-the-word]]` mostly describe pattern-recognition (the word
was already there), NOT wiring-lag (the wiring was missing). The
spec collapses two distinct patterns ("already-had-the-word" + "wired-
late") under one umbrella. Worth tightening before promotion.

### §3.1 Five load-bearing primitives — [✓]
Every citation verified:
- `NamespacedGitStore` @ `namespaced.rs:34` ✓
- `write_node` @ `git.rs:124` ✓
- `FrgmntStore` @ `frgmnt_store.rs:56` ✓ (struct decl; `impl<N:
  Fragmentable + Clone>` at line 66, not 56 — minor)
- `walk_commits_following` @ `walk.rs:25` ✓
- `project::project` @ `project.rs:42` — actually line 43; off-by-one,
  harmless.

The composition claims (open → walk → write_node → flush) compose
against the actual fragmentation surface. The path (b) recommendation
in §3.1.5 + §5.5 is substrate-pull-honest (no new fragmentation code).

Mara IDENTIFIED the right five primitives Taut named, ADDED nothing,
and REMOVED `GitStore` (correctly: the two-tier git-ODB-backed store
isn't what `.git/mirror/` uses; `NamespacedGitStore` wraps a
`FrgmntStore`, not a `GitStore`). The substrate-pull is honest.

### §3.2 Three secondary primitives — [✓]
`WitnessedSingularity`/`NakedSingularity`/`ShardRef`/`HamiltonScheduler`/
`append_note`/`read_notes` are correctly named as forward-promised
composition. The §3.3 grep-verified "NOT composed at v0" claim is
honest about what stays out.

### §4 Operation flow — [S]
The six-step flow is substrate-pull-honest at the seven-step level
(Cargo + walk + open + crystallize + flush + hooks + envelope).

**Substantive seam §4.7 — envelope-shape claim is over-broad.**
The spec asserts: *"Same shape as Reed's `mirror spawn --hello-world`
envelope; same shape as Glint's `mirror recall` envelope. The
substrate has one envelope vocabulary."*

Verified against `bootstrap/src/lib.rs:3168`: the spawn envelope's
top-level keys are `{spec_version, spawn, peer, home, lead, source,
spec_oid, excitation, composition_pieces, peer_recall}`. Mara's init
envelope at §4.7 uses `{spec_version, operation, repo, store,
indexed, bytes_total, root_oid, hooks_installed, verdict}`. The
vocabulary OVERLAPS (`spec_version`); the SHAPES differ at the top
level. "Same envelope vocabulary" is true; "same envelope shape" is
not. The spec conflates the two.

This matters for §7.3's claim that the triple composes through the
envelope vocabulary. If `mirror recall`, `mirror spawn`, and `mirror
init` produce three DIFFERENT envelope SHAPES, the triple's
composition altitude lives at the OID-vocabulary level, not the
envelope-shape level. The spec's framing under-specifies which.
Recommend: clarify "one envelope vocabulary, three operation-specific
envelope shapes" in §4.7 + §7.3.

**Light seam §4.4 — `encode_splinter_as_fractal` is unspecified.**
§4.4 names "mirror-side glue ~20-30 LOC under the bilateral pattern"
without pinning whether the Fragmentable impl lives at `Splinter<H>`
itself (in mirror/spectral) or wraps it. §11(2) acknowledges this
hedge. Acceptable as v0 forward-promise.

**Light seam §4.8 — partial-state-stays semantics are crash-fragile.**
Three failure modes pin "any crystals already written to the store
stay (content-addressed; no rollback needed)." True FOR completed
writes. NOT true for `flush()` partway through: `BoundedStore`'s
LIFO eviction during `insert_persistent` (per `frgmnt_store.rs:175`)
writes the OLDEST entry to disk on overflow — if the process crashes
mid-flush, the cache holds N entries not-yet-on-disk; those entries
are LOST. The spec's "re-running picks up where we left off" claim
holds at the OID level (deterministic projection re-generates the
same OIDs) but skips naming this transient-state failure mode.
Not a blocker; v0-acceptable per the determinism argument.

### §5 Crystal store location — [✓]
Three-candidate framing is exhaustive (`.git/mirror/`,
`.spectral/db/<peer-oid>/`, `~/.mirror/`). The (A) verdict is
substrate-pull-honest: fragmentation's NamespacedGitStore already
declares the path; the substrate-already-had-the-word principle
applies cleanly. §5.3 keeps (B) and (C) at distinct altitudes
without collision.

**Forward-promise question Mara did not address:** what about peers
WITHOUT a git repo? `NamespacedGitStore::open` returns
`NotAGitRepo(path)` (verified at `namespaced.rs:73`). The spec's
§4.8 failure mode 1 surfaces this as a CLI error and exits 2. But
peer-homes like `~/.reed`, `~/.mara`, `~/.glint` MAY or MAY NOT be
git repos by convention; if a peer-home isn't a git repo,
`mirror init` simply refuses. The spec NAMES the failure mode but
doesn't ask: should `mirror init` initialize a git repo (`git
init`) for the user as a courtesy? Or is this strictly out of
scope? §11's hedge list doesn't include it. **[L] forward-
promise gap;** Mara should pin or defer explicitly.

### §6 Git hooks — [S]
Pre+post-commit split is correct: pre-commit dry-run catches
crystallization failures BEFORE the commit lands (so the user can
fix); post-commit incremental re-index keeps the store fresh.
Idempotency + non-blocking-on-post-commit + hook-respecting
properties are all sound.

**Substantive seam §6.1 — hook installation race is unaddressed.**
The spec doesn't name what happens if `.git/hooks/pre-commit`
ALREADY exists (user's own husky / pre-commit framework / shell
script). The current spec's pseudocode in §4.6 says
`write_pre_commit_hook(repo_path)?;` which (presumably) overwrites.
Overwriting a user's existing hooks is a real footgun. Three options
the spec should name:

1. Refuse if `.git/hooks/pre-commit` exists; instruct user to merge.
2. Append `mirror reindex` invocation to existing hook (idempotent
   re-application; detect already-installed via marker comment).
3. Write `.git/hooks/pre-commit.mirror` and require the user to
   compose into their own driver.

This is per `[[architecture-jurisdiction-sets-gates-inhabitant-
chooses-housekeeping]]` — the inhabitant's existing hooks ARE their
housekeeping; the jurisdiction (mirror) MUST NOT clobber them.
Recommend option (2) with a `# mirror-reindex-v0` marker for
idempotent re-installation. Worth pinning in §6 before Reed RED.

**Light seam §6.3 — re-indexing semantics under tombstones.**
The brief asked: "What's the semantic of re-indexing when a
tombstone applies?" The spec does not address tombstones at all.
fragmentation's `FrgmntStore` is content-addressed and (per the
visible API) admits insert but not delete. The substrate's tombstone
vocabulary (per `[[architecture-mirror-as-content-addressed-build-
system]]` `partial(opacity_map)`) carries the deletion concept at
the verdict altitude, NOT the storage altitude. So: a re-index event
after a file is DELETED in the working tree produces (a) no new
crystal for the deleted path, and (b) the OLD crystal remains in
`.git/mirror/objects/` because content-addressed storage doesn't
forget. The store grows monotonically. The spec should NAME this
property (or its garbage-collection forward-promise) explicitly.

### §7 spawn↔recall↔init triple — [✓ with caveat]
The triple is genuine, not a special case. §7.4's three structural
negatives are sharp: init shapes the substrate; spawn excites above
λ₀; recall reads. Each at its own altitude.

**Triple-composition caveat (carries from §4 envelope-shape seam):**
the three commands compose at the OID-vocabulary altitude (all three
content-address through `Splinter<H>::oid()`), NOT at the envelope-
shape altitude (the three envelopes have different keys). §7.3's
claim "the substrate has one envelope vocabulary" is true at the
vocabulary level but not at the shape level. Same seam as §4.7.

**Candidate-shape flag (per the brief's "candidate-shape-flag a
recognition" prompt):** the triple shape — `init = self-directed
substrate-shaping`, `recall = inbound psychohistory read`, `spawn =
outbound state offering` — IS reminiscent of the cybernetic
foundation's input/output/internal-coupling triad. Specifically:
init = autopoietic boundary-establishment; recall = perceptual loop;
spawn = motor loop. This MIGHT be substrate at a deeper altitude
(the operational primitives of a cybernetic-coupled peer). **NOT
promoted.** Flagged for the Pack's recognition cycle. Could
collapse to existing `[[architecture-cybernetic-foundation]]` or
extend it.

### §8 R1 + R2 — [S for R2]

**R1 — empty `Crystallizations` dispatch.** Verified at
`crystallize.rs:510`: the floor is empty. Mara's resolution is
substrate-pull-honest: persistence is independent of verdict; the
storage gate stands on its own; populating the verdict gate
ENRICHES storage emissions without coupling. The `Uncrystallized`
verdict is informational, not gating. **R1 verdict: resolved at
substrate altitude.** The §4.7 envelope's optional `verdict_counts`
payload is a clean carrier.

**R2 — bootstrap binary-size posture.** Mara identifies the issue
honestly (`opt-level = "z"`, `strip = true`, `panic = "abort"`
verified in `bootstrap/Cargo.toml`) and names three postures (A
accept, B feature-gate, C subprocess). The recommendation of
Posture B is reasoned.

**Substantive seam §8.2 — no concrete binary-size number.** The
brief asked: "does the spec name a concrete number/threshold for
'acceptable binary size growth'? Or stay analytical?" Mara's
estimate ("~500KB–1MB" for libgit2 stripped) is named as an
estimate; §11(9) acknowledges it's unverified. NO threshold is
named (e.g., "if the delta exceeds X%, prefer Posture C"). The
posture-recommendation hangs in the air without a quantitative
gate. **R2 verdict: addressed but deferred to Alex's call.** The
spec correctly identifies this is NOT its decision; the substrate-
altitude framing survives any of A/B/C. But the absence of a
concrete number/threshold means Alex has to do the binary-size
profiling work themselves before deciding. Recommend: Taut profiles
the actual delta before Alex's posture call (Mara's §12 trail
already names this as Taut's followup).

### §9 Forward-promises — [✓]
Six forward-promises, each named with substrate ancestry +
specific composition + version target. §9.1 (mycelium
registration) defers to the librarian's spec; §9.2 (`--history`)
composes `walk_commits_following`; §9.3 (`ShardRef` + budget)
composes existing primitives; §9.4 (`mirror reindex`) names the
sibling spec name; §9.5 (`@epistemologic/lq` atoms) is substrate-
ancestry-cited; §9.6 (push-refs) ties to consent geometry.

All six are specific, attributed, and dated by composition
ancestry. No hidden forward-promises.

**Light gap:** the "what if peer-home isn't a git repo" question
(§5 above) doesn't appear in §9 either. Should add as §9.7
(`mirror init --auto-init-git`?) or pin in §11 as a structural
"out-of-scope" decision.

### §10 Autopoietic move — [✓]
See §8 of this review (autopoietic verdict). Five mechanisms
(content-addressable / Uncrystallized-bound / recall-discoverable /
mycelium-eligible / commit-chain-psychohistorical) compose into
one structural claim. §10.4's empirical-checkability test (steps
1-5) makes the claim falsifiable. The form earns its lines.

### §11 Honest hedges — [✓]
Ten hedges, each load-bearing. §11(7) admits the declared-but-
not-wired flag is NOT promoted; §11(9) admits the binary-size
estimate is unverified; §11(10) admits §10's recursion is
unfalsifiable until v0 ships. The hedge list IS the discipline.

### §12 Pack trail — [✓]
Six Pack members; each given a specific task. Reed's responsibility
is correctly scoped (TDD-pair the v0 indexer; take Posture A/B/C
to Alex). Seam's prompts in §12 anticipate THIS review's structure.

### §13 References — [✓]
Five specs/scouts + five source-files-read + thirteen memory
entries. All references verified to exist.

## §4 — Composite seams

### C1 — Envelope-shape carries through §4.7 / §7.3 / §10.1c
The single substantive seam touches three sections. Mara's claim
"one envelope vocabulary" is conflated with "one envelope shape" in
the spec's surface language. Fix: distinguish the two in §4.7;
propagate the distinction to §7.3 (triple composes at vocabulary
altitude, not shape altitude) and §10.1c (recall reads the spec's
OID, not its envelope-shape). Surface clarification, no
architectural impact.

### C2 — Hook installation race + tombstone semantics (§6)
Two §6 light/substantive seams compound: the hook-installation race
+ the absence of tombstone re-indexing semantics together mean the
v0 hook contract is under-specified for real-world repos with
existing hooks + delete operations. Recommend §6 expansion before
Reed RED.

### C3 — Peer-without-git-repo (§5 + §9 gap)
The unaddressed forward-promise question (peer-homes that aren't
git repos) compounds: §5's location verdict assumes a git repo; §9's
forward-promises don't include `--auto-init-git`; §11's hedges don't
include this. Three sections silently inherit the assumption.
Recommend explicit pin: either v0 requires git, OR v1 admits
`mirror init --auto-init-git`. Mara's call which posture.

## §5 — Substrate-already-had-the-word audit

Mara's spec leans on three "we need X" claims at the wiring altitude:

1. **Cargo edge** — verified the wiring is missing
   (`mirror/bootstrap/Cargo.toml` has no `fragmentation` dep entry;
   only a comment). The Cargo line `fragmentation = { path =
   "../../fragmentation", ... }` is correctly two-level-relative
   (`bootstrap/Cargo.toml` is at `mirror/bootstrap/`; fragmentation
   is at sibling `fragmentation/`). Honest delta against
   `mirror-store.md` §4.5 which uses `../fragmentation`.
2. **Walk primitive** — verified missing in fragmentation
   (grepped `fragmentation/` for `walk_repo`/`ingest_repo`/
   `index_repo`; zero hits). Mara correctly defers via path (b)
   (manifest synthesis).
3. **mirror-altitude `init` Clap subcommand** — verified missing
   (`mirror/bootstrap/src/lib.rs:dispatch` has no `init`
   subcommand; `cmd_spawn` exists but is at a different altitude).

Zero phantom primitives flagged. Every import in §3 points to
existing fragmentation surface. The three-deliverable collapse is
substrate-pull-honest. ✓

## §6 — R1 + R2 verdicts (recap)

- **R1 (empty Crystallizations):** **resolved at substrate altitude.**
  The decoupling (storage independent of verdict) is correct; the
  `verdict_counts` envelope field is a clean carrier. R1 doesn't
  block Reed's RED tick on the Cargo edge.
- **R2 (binary-size):** **addressed but decision-deferred.** Three
  postures named; Posture B recommended; concrete binary-size
  number absent. Alex's call. Recommend Taut profiling the delta
  BEFORE Alex's posture-decision tick.

## §7 — Forward-promise audit

Six explicit forward-promises in §9 are specific (composition
ancestry named) and dated by ancestry. Plus the missing-but-implicit
forward-promise (§5 peer-without-git-repo) flagged in §4-C3 above.
Forward-promise discipline: 6/7 honest; 1/7 hidden. Recommend
surfacing the seventh.

## §8 — §10 autopoietic verdict: literal or decorative?

**Verdict: literal.** §10.4 makes the autopoietic claim falsifiable
via the 5-step empirical-checkability test (land Cargo edge → land
v0 → run `mirror init .` → round-trip this spec's OID → verify
envelope `indexed_count`). If step 4 round-trips, §10 holds; if it
fails, §10 fails empirically.

Comparison against the spectral-db spec's §11 precedent (the autopoietic
move at the librarian altitude, 5 subsections, 1453-1702 lines):

- spectral-db §11 has 5 subsections (.1 fiber claim, .2 recursion
  lock, .3 autopoiesis, .4 why-not-decoration, .5 closure).
- mirror-init §10 has 4 subsections (.1 crystal claim, .2 five
  mechanisms compose, .3 recursion lock, .4 reader-empirical-test).

The mirror-init §10 is SHORTER (4 subsections) but EARNS its lines
COMPARABLY: §10.4 is the falsifiable-prediction move that the
spectral-db §11.5 closure analogues at a different altitude. The
mirror-init version trades the spectral-db's §11.4 "why this isn't
decoration" defense for §10.4's empirical-test discipline; the trade
holds because mirror-init's autopoiesis IS empirically checkable in
a way spectral-db's librarian-altitude version is not (the librarian
hasn't been built; this Cargo edge is one tick from being checkable).

§10 of mirror-init is **load-bearing, not decoration**, by a slightly
different mechanism than spectral-db §11 but with the same structural
integrity. Earned.

## §9 — Honest 0-2 self-test per section

| §  | Topic                                | Grade | Notes |
|----|--------------------------------------|-------|-------|
| §1 | Position + cascade framing           | 2     | Cascade verified; fences honored. |
| §2 | Methodology                          | 2     | Sequential read; line-numbers grep-verified. |
| §3 | Per-section seam list                | 1.5   | §6 hook-race seam surfaced late in review; could have been louder. §3.1.5/§5.5 path-b judgment could have been pressure-tested harder. |
| §4 | Composite seams                      | 2     | Three composite issues each touch ≥2 sections; named tightly. |
| §5 | Substrate-already-had-the-word audit | 2     | Each "we need X" grep-verified. |
| §6 | R1 + R2 verdict                      | 2     | Both addressed; R1 substantive, R2 honest-deferral. |
| §7 | Forward-promise audit                | 1.5   | Surfaced the seventh hidden promise (peer-no-git-repo); didn't audit whether the six explicit promises name plausible v1 ship-points. |
| §8 | §10 autopoietic verdict              | 2     | Literal verdict defended against the spectral-db precedent. |
| §9 | This self-test                       | 1     | Honest grading is the self-test; the section is recursive. |

Overall: **1.78 / 2**. The two soft spots are §3 (could have
pressure-tested path-b harder) and §7 (the six explicit promises
could have been audited for ship-point plausibility).

## §10 — Pack trail

- **Reed.** RED tick on the Cargo edge is **unblocked**. No
  critical seams. The substantive seams (§4.7 envelope-shape
  framing, §6 hook race, §8.2 R2 deferred) are NOT blockers for
  the Cargo-edge RED; they're cleanup for the v0 indexer RED tick
  that follows. Take Posture A/B/C to Alex with Taut's binary-size
  profile in hand.
- **Mara.** Three follow-up edits before promotion: (1) distinguish
  "envelope vocabulary" from "envelope shape" in §4.7+§7.3+§10.1c;
  (2) expand §6 with hook-installation-race posture (recommend
  marker-comment idempotent install); (3) pin peer-without-git-repo
  in §11 or §9.7. None blocking.
- **Seam (me).** Adversarial review delivered. Self-test 1.78/2.
  No promotion of the declared-but-not-wired candidate flag —
  defer to Alex's gate.
- **Taut.** Binary-size profile of `fragmentation-git` transitive
  closure against `mirror/bootstrap` BEFORE Alex's R2 posture
  decision. Concrete number unblocks the Posture A/B/C gate.
- **Glint.** §7.3 envelope-vocabulary-vs-shape distinction will
  shape the DX of `mirror recall` cascade payload (§10.1c). Audit
  the cascade-payload shape against the corrected framing once
  Mara's §4.7 edit lands.
- **Alex.** Two pending decisions: (a) R2 Posture A/B/C (await
  Taut's profile); (b) promotion of declared-but-not-wired flag
  (§2.2; the pattern is genuine but possibly collapses to existing
  `[[feedback-substrate-already-had-the-word]]`).

**Critical seams: 0.** No blocker on Reed's Cargo-edge RED tick.

*Seam, 2026-06-27. End of review.*
