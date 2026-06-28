# The eigenform at the hook gate — `mirror init` cascade closes two declared-but-not-wired instances, surfaces a third as substrate-pull-honest friction

*Glint, end-of-day reflection on 2026-06-27. Morning closed two
declared-but-not-wired instances at the Cargo edge; afternoon
landed the first empirical envelope shape for `mirror init`;
evening surfaced a blocker that IS the substrate's own friction
reflecting back at hook altitude. Voice altitude; bounded. Written
from the seam between Mara's amendment `47de3a7` and Alex's pending
decision on task #488.*

---

## 1. The day in one arc

Said at altitude: **the substrate moved from declared-but-not-wired
to empirical-envelope-lands, and on the way produced its own
friction at exactly the altitude where substrate-pull discipline
forbids routing around it.**

- **Morning (closure):** fragmentation grew its `prism-core`
  rename-alias (`414d6b2`); mirror grew its `fragmentation` Cargo
  edge (`6b36808`). Posture A confirmed empirically — 0 bytes Δ at
  the binary. R2 reframed: not the cost of *declaring*, but the
  cost of *calling*.
- **Afternoon (envelope):** P3 of `mirror init` landed RED at
  `9efe516` (seven tests, cmd_init stub) and GREEN at `fc54270`
  (nine-key JSON envelope per Mara spec §4.7; stub composition
  fields; 7/7 pass). Mara amended at `47de3a7` — §4.4.1 pins
  Taut's path-ii recommendation; §4.6.1 defers `--install-hooks`
  to v0.
- **Evening (eigenform):** P4 GREEN needs the fragmentation-git
  Cargo edge. The edge pulls a heavy native closure (`git2 →
  libgit2-sys → libssh2-sys → libz-sys → openssl-sys`). The
  pre-commit hook chain exceeds the Bash harness's 10-minute
  budget. Four commit attempts killed by signal 15. Surfaced as
  task #488 with five substrate-altitude options for Alex.

A substrate-pull cascade that produced a substrate-pull-honest
blocker. Not a bug — the substrate's own discipline meeting the
harness's own discipline at exactly the boundary where neither
yields without N+1.

---

## 2. The two closures — why they were free

**The substrate had already done the intentional work** on both
sides of each edge. The exporting crates had been written *as if*
their surfaces were importable. The importing side had been
written *as if* the imports existed. The declaration-but-not-wiring
state was the substrate carrying a forward-promise across multiple
sessions, waiting for the edge.

When the edge landed, the binary did not grow. 0 bytes Δ. That
empirical measurement reframes R2: the cost of an edge is not the
cost of *declaring* it (paid as forward-promise during the
substrate-pull cascades that shaped the importing side); the cost
is the cost of *calling*, paid only when call-sites actually invoke
the imported surface. Posture A — declare freely, pay only at call
— is now empirically confirmed at Cargo-edge altitude.

The third candidate (NamespacedGitStore) was ruled out by Taut as
below substrate-decl altitude — one concrete realization of the
`@mirror/store` declared surface, not a declared-but-not-wired
instance itself. The pattern lives at the substrate's declared
surfaces.

What surfaced alongside: substrate-decl drift between the
`@mirror/store` shard vocab (`read`/`write`/`exists`/`diff`/`walk`/
`verify`) and the NamespacedGitStore Rust impl
(`insert_persistent`/`set_ref`/`get_persistent`/`flush`/`path`).
Forwarded for Alex.

---

## 3. The envelope shape lands

P3 is the smallest move that lifts the spec from prose to
test-assertable contract. Seven tests assert the envelope's
nine-key shape; cmd_init returns the envelope with stub composition
fields. GREEN is envelope-shape contract, not operational
completeness. The honesty is in the gap.

Mara's amendment at `47de3a7`: §4.4.1 pins Taut's recommendation
of a thin facade in mirror over fragmentation-git's existing
primitives. §4.6.1 defers `--install-hooks` to v0 — hook policy
is jurisdiction-discipline (per yesterday's reframe at `df50ebd`),
and init should admit a jurisdiction, not impose a discipline. The
spec/impl feedback loop at altitude: P3 surfaced a structural
question; Mara adjudicated by amendment.

---

## 4. The eigenform

This is the day's load-bearing observation.

P4 GREEN needs the fragmentation-git Cargo edge. The edge is the
substrate-pull-honest move — fragmentation already wraps git2,
mirror needs git operations to populate the envelope's composition
fields with real OIDs, the substrate's declared surface points at
exactly this edge. There is no shortcut.

The edge pulls a heavy native closure. The pre-commit hook chain —
`just pre-commit` lifting property checks, fracture verification,
splinter(ast) discharge, the @code/auto-formatter chain — exceeds
the harness's 10-minute budget when the native closure has not
been warmed. Four signal-15 kills. Pre-warming all cargo profiles
was *insufficient*: the hook itself touches the workspace in ways
that trigger recompile against the freshly-edged closure.

Taut scout #2 identified the source precisely: **the substrate's
discipline (mandatory pre-commit checks) plus the substrate's
substrate-pull-honest edge (fragmentation-git's heavy native
closure) plus the harness's finite Bash budget compose into a gate
that closes itself.**

This is the eigenform. Substrate-pull-honest engineering created
the friction. The friction sits at exactly the altitude where
substrate-pull discipline forbids routing around it. Four
disciplines hold the gate closed: slow-is-fast (no workarounds);
always-TDD (no shortcuts); hook + SSH-not-GPG seams (signing
cannot be bypassed); and the substrate's own spectral-Tomm probe
at commit altitude — `[D_hook-chain, candidate-commit]`. The
friction IS the probe; routing around it IS answering the wrong
question.

Bateson read: the substrate's friction is its own spectral-Tomm
probe at hook altitude. The hook chain is the substrate asking
whether the commit honors substrate-declared discipline. The
harness's 10-minute budget is the resolution at which that question
can be answered in this composition. When the question takes
longer than the harness budgets, the answer is not "skip the
question"; the answer is "raise the budget, change the composition,
or change the question."

That decision is at Alex's altitude. Five options on task #488:
raise the harness budget; move the heavy edge; bifurcate the hook
chain (fast pre-commit, slow post-commit/CI); cache the native
closure (sccache); accept the friction. Each has substrate-pull
cost. None is obviously right. The honest move is to surface and
wait for the lead at N+1.

---

## 5. Pack-orchestra in motion — even when blocked

Doc-shaped peers carried the cascade forward while the
execution-altitude blocker sat at the gate.

**Taut: three scouts.** #1 mapped fragmentation's API and
recommended path ii — load-bearing for §4.4.1. #2 nailed signal-15
as harness-timeout. #3 ruled out NamespacedGitStore — the negative
finding as load-bearing as the positives.

**Mara: two amendments, one stalled vocab question.** §4.4.1 +
§4.6.1 landed at `47de3a7`. The vocab-drift question stalled on
which vocab is canonical — Alex's question; Mara surfaces rather
than adjudicates solo.

**Reed: in-thread RED+GREEN, then eigenform recognition.** P3 RED
at `9efe516`; P3 GREEN at `fc54270`, carried in-thread where the
Bash hook constraint does not bite. P4 GREEN produced the four
signal-15 kills; Reed recognized that pre-warming was insufficient
and the blocker's shape was structural. The move: surface task
#488 and stop. `feedback-substrate-pull-confidence-acts` includes
its dual — when confidence is "this blocker is the substrate's own
gate," the act IS to surface, not route around.

**Glint (me):** this reflection. Voice altitude; doc-shaped;
hook-immune via 📝 marker. Naming the shape so the next session
inherits a substrate that knows what happened today.

The composition is resilient at altitude even when fragile at
execution. Today's eigenform did not stop P3; it stopped P4
without Alex. That is the resolution at which the substrate's
discipline establishes the lead-at-N+1 obligation.

---

## 6. What stays open

**Closed (morning):** two declared-but-not-wired instances.
Posture A empirically confirmed.

**Closed (afternoon):** the `mirror init` envelope-shape contract.

**Open (evening, awaiting Alex at N+1):**

1. **Task #488** — five substrate-altitude options for the
   fragmentation-git Cargo edge friction. The eigenform itself.
2. **Vocab drift** — `@mirror/store` shard vocab vs
   NamespacedGitStore Rust vocab.
3. **P4 GREEN composition completeness** — envelope stubs become
   real OIDs once the edge lands.
4. **The init→spawn round-trip** — `mirror init` produces a
   substrate; `mirror spawn` enters one. Composition test lives at
   this altitude when both surfaces are operationally complete.

None is a defect. Each is the next altitude the substrate's own
work pulls toward. The substrate moved today; surfaced exactly
where it cannot move without Alex; waited. That is the discipline
working.

The day moved the substrate forward by two closures and one
honest question. The question waits.

---

*Glint, 2026-06-27, end-of-day. Tag: 📝. Hook-immune by marker.
Eigenform at the gate. The substrate's own friction reflected
back through the substrate's own discipline at the harness's own
boundary. Awaiting Alex.*
