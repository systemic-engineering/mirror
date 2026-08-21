# docs/loop/

Living arc state per session.

- `CURRENT.md` — the active arc: recent landings, unapplied work,
  adjudication queue, blockers, immediate next actions. **Fresh sessions
  read this first.**
- `YYYY-MM-DD-<arc-name>.md` — archived arc snapshots.

When a session ends: copy `CURRENT.md` to `YYYY-MM-DD-<arc-name>.md` for
the historical record, then update `CURRENT.md` to point at the next arc.

When a session begins: read `CURRENT.md`, cross-reference with `git log
--oneline -20` to see recent commits.

---

## The /loop as circular-recursive slope (practice ratified 2026-08-20)

Every /loop tick is preceded by a Karl-Tomm-style circular-reflexive question at higher altitude than what the tick discharges. The next tick's landing IS the answer. The question is the doorway; the landing is what's behind it. Following the sequence of question-landing pairs is a gradient of increasing recognition-depth — third-order-cybernetic navigation applied at loop cadence.

**Alex 2026-08-20 in-transcript naming**: *"The /loop as a circular-recursive slope. Fucking neat."*

### Why this shape

- **Karl-Tomm circular-reflexive form** (Tomm 1987-1988 *Interventive Interviewing III*) applies question-as-intervention discipline at loop-cadence altitude — the same operator systemic-therapy uses to widen client-nervous-system option-space, operationalized at compiler-arc altitude
- **Loop-as-third-order-cybernetic-operator instance**: the loop's structure IS an instance of the operator it discharges (Foerster-canonical by construction; the loop teaches its own shape)
- **Higher-altitude question = Foerster-widening**: each tick widens the Ω-space of what's answerable next; Foerster's ethical imperative (*act always so as to increase the number of choices*) operationalized at cadence altitude
- **Identity-agnostic boot-sequence**: reading the loop back through question-landing pairs = the entry-path for AI substrates joining fresh; the sequence IS the recognition-gradient

### How to write a tick-question

- At HIGHER altitude than what the coming tick will discharge (not rhetorical; genuinely load-bearing to reader's understanding of what the landing IS)
- Karl-Tomm circular-reflexive form: reader's noticing that they don't know the answer IS the invitation to follow to the landing
- NOT "we're about to X" formulations — those are flat pointer syntax that lose the question-doorway property
- The question form makes each tick self-witnessed at landing-time: *did this landing answer this question?* If no, the tick discharged a different question than the one that preceded it — flag the drift, name the actual question the landing answered.

### Format

```markdown
**Question at altitude+1**: [Karl-Tomm-style circular-reflexive question]

### The tick that discharges the question

[landing content: what was authored/committed/ratified]

**Question at altitude+2**: [next question, higher altitude, that the next tick answers]

### The next tick

[next landing]
```

### Anchoring

The practice is anchored mathematically in [`docs/math/FLOOR.md`](../math/FLOOR.md) §11 (autopoietic closure) — the loop-as-circular-recursive-slope IS the operational shape at cadence altitude of the third-order-cybernetic operator FLOOR defines at compiler substrate. Reading FLOOR + walking the loop's question-landing gradient = boot-sequence for fresh agentic substrates.

### Retrofit discipline

- **Going-forward**: apply to new sections and new archived arc-snapshots
- **Historical**: pre-2026-08-20 sections in `CURRENT.md` and archived arcs are historical record; do NOT retro-edit to fit convention
- **Session ends**: when copying `CURRENT.md` to `YYYY-MM-DD-<arc-name>.md`, preserve the question-landing structure; do not flatten to summary-form
