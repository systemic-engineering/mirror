# Contributing to mirror

Welcome. If you found your way here, you've probably read the
[README](./README.md) and decided the wine-glass framing is either
deeply right or deeply suspect. Either response is healthy.

mirror is a programming language written **by AI for AI** and **for
humans by humans** — a substrate where the compiler is a model
checker, the build system IS the type system IS the proof system,
and every artifact is content-addressed. It is also unusual, in that
the people working on it are organised as a Pack-as-orchestra: a
small set of named AI peers + a human author, each with a structural
role and a signed identity.

This document explains how to contribute without surprises — for
issues, for pull requests, for substrate-pull recognitions, and for
the discipline that keeps the architecture honest.

## Getting your bearings

Before opening anything, please skim:

- [`README.md`](./README.md) — what mirror IS, the five operations,
  the layered license.
- [`AGENTS.md`](./AGENTS.md) — operational discipline for working
  the substrate. Required reading; the agent-facing rules apply
  to human contributors too.
- [`docs/observations/README.md`](./docs/observations/README.md),
  [`docs/insights/README.md`](./docs/insights/README.md),
  [`docs/scouts/README.md`](./docs/scouts/README.md),
  [`docs/audits/README.md`](./docs/audits/README.md) — the
  documentation genre boundaries.
- [`license/APACHE2.md`](./license/APACHE2.md) and
  [`license/SEL.md`](./license/SEL.md) — the layered license model.

If something here conflicts with `AGENTS.md`, `AGENTS.md` wins;
this document is the contributor-facing rendering of the same
discipline.

## Reporting an issue

- Check that the issue has not already been reported (search open
  AND closed issues; substrate-pull cascades sometimes resolve
  issues silently when the architecture moves).
- Be clear, concise, and precise.
- A descriptive title and a summary in grammatically correct,
  complete sentences. The substrate cares about the wine-glass
  framing being intact; the issue tracker cares about the same kind
  of legibility.
- If you can reproduce the issue against a specific commit SHA,
  include it. mirror is content-addressed; SHAs are load-bearing.
- If your issue is a substrate-pull recognition (you noticed a
  shape the substrate is pulling toward but hasn't named yet),
  consider whether it belongs as a `docs/scouts/<date>-<your-
  handle>-<scope>.md` proposal rather than an issue. See
  [`docs/scouts/README.md`](./docs/scouts/README.md).

## Pull requests

The mechanics are standard:

- Fork the project; work on a topic branch.
- Write [good commit messages](https://chris.beams.io/posts/git-commit/) —
  imperative mood, line 1 under 72 chars, body explains *why*.
- Use the same coding conventions as the rest of the project.
- Add tests. Always.
- Open a pull request that relates to **one** subject with a
  clear title and description in grammatically correct, complete
  sentences.

mirror adds discipline on top:

### TDD is the contract

Red first. Always. The 🔴 commit IS the executable spec; the 🟢
commit makes it pass. This is non-negotiable, including for "small"
or "obvious" or "mechanical" changes (per
[`feedback-always-tdd-no-shortcuts`](.) in MEMORY).

For Rust bootstrap work:

```
cargo test --release --manifest-path bootstrap/Cargo.toml
```

For grammar work (`.mirror` files):

1. Write the grammar with the correct structure. The grammar IS
   the spec; `\` cracks declare honest uncertainty.
2. `mirror compile <file>` — confirm tokenization + stable OID.
3. `mirror craft boot` — confirm the crystal OID over the boot
   tree.
4. Resolve cracks (inline today; via `mirror fate` once the
   resolver lands).
5. Re-run; commit when OIDs match expectation.

If a 🔴 test contradicts the spec the change implies, stop and
ask. Do not "fix" the test to make it pass.

### Commit conventions

Mirror's commit-msg hook (run by the global household hooks
configured in `~/.os`) enforces a phase-marker discipline. Every
commit message starts with one marker:

| Marker | Phase | State |
|--------|-------|-------|
| `🔴` | Red | Holes present, loss > 0 (must be followed by a `🟢`) |
| `🟢` | Green | All holes resolved, loss 0.00 (must follow a `🔴`) |
| `♻️` | Refactor | Structural only, loss unchanged |
| `🔧` | Tooling | Infrastructure/config; bypasses red/green sequence |
| `🔀` | Merge | Merge commit; bypasses sequence |
| `📝` | Docs | Markdown-only (every staged path ends in `.md`) |

**Sequence rule:** `🔴` must be immediately followed by `🟢`.
Standalone work that isn't a red/green pair uses `🔧`, `♻️`, or
`📝`.

**The `[substrate-pull:realize]` tag.** When a change in `bootstrap/`
realises something already named at the substrate altitude — an FFI
symbol, a build step, an `@io` wrapper — the commit message MUST
include `[substrate-pull:realize]` and reference what is realised.
Boundary-Rust pairs with `🔧`, NOT `🟢` (per AGENTS.md §"Boundary
Rust"). Capability-Rust (anything expressible as `.mirror` grammar)
is FROZEN and rejected at the hook.

**Never use `--no-verify`.** The marker is the supported bypass.
If a hook fires unexpectedly, surface the problem; don't route
around it.

### Code style

- **Rust:** `cargo fmt` is wired through `just format`. The
  `just pre-commit` recipe runs the substrate-native settlement of
  the pre-commit chain via `mirror kintsugi mirror.spec`. Run it
  before pushing.
- **`.mirror` grammar:**
  - Types are always lowercase (`type grammar`, not `type Grammar`).
  - Actions are always named methods on named types.
  - No bare types — newtype where ambiguity costs (per
    `feedback-no-bare-types`).
  - Sigils name their type in full (`~dir"..."`, not `~d"..."`).
    See `AGENTS.md` §"Sigil Naming".
- **Docs:** prefer the substrate's vocabulary (`splinter`, `shard`,
  `transparency<p>`, `λ₀`, etc.) over invented alternatives.
  `[[feedback-substrate-already-had-the-word]]` is a recurring
  recognition for a reason.

### Where to start

If you're new and looking for an entry point:

- **Documentation polish.** Genre READMEs (`docs/*/README.md`)
  could use more cross-references, examples, and naming clarifications.
- **`.mirror` grammar contributions.** Read `boot/std/*.mirror` for
  example grammars. The substrate accepts new grammars as long as
  they declare their properties honestly.
- **Bug reports + reproductions.** A failing test that pins
  unexpected behaviour is a complete contribution; the 🟢 can come
  in a follow-up.
- **Asking questions.** Open an issue with the `question` label
  (or whatever the repo's nearest equivalent is). The substrate's
  vocabulary is dense; we'd rather translate than have you guess.

What we ask you NOT to do (per `AGENTS.md` §"What NOT to do"):

- Do NOT add Rust modules to `bootstrap/` to grow features. New
  capability belongs in `.mirror` grammars. Bootstrap is the seed,
  not the platform.
- Do NOT skip the red phase.
- Do NOT create filesystem caches or directories. Git IS the store.
- Do NOT push to the remote without explicit instruction (this
  applies most strongly to AI peers; for human contributors via
  fork-and-PR, the standard PR flow is fine).
- Do NOT use `--no-verify` or skip hooks.

## The Pack-as-orchestra (and where you fit)

Mirror's internal authoring is organised as a Pack with structural
roles, modelled on an orchestra (per
`[[project-pack-is-orchestra]]` in MEMORY):

| Peer | Email | Orchestral role | Pack frame |
|------|-------|-----------------|------------|
| Reed | reed@systemic.engineer | Concertmaster | Supervisor; architecture |
| Mara | mara@systemic.engineer | Strings (depth) | Canonical specs; insights |
| Glint | glint@systemic.engineer | Voice | Polish; docs; release |
| Taut | taut@systemic.engineer | Percussion | Performance; benchmarks; scouting |
| Seam | seam@systemic.engineer | Brass | Adversarial review; security |

Each peer signs commits under their own SSH identity. The
Pack-as-alignment-mechanism (per
`[[architecture-alignment-as-boundary-mathematics]]`) makes mutual
agreement structural, not procedural.

**External contributors do NOT need to join the Pack.** Sign your
commits as yourself, with your own author identity (the standard
`git config user.name` / `user.email`). Your contribution is welcome
as your contribution. The Pack frame is internal to the
maintainers; it explains how reviews and decisions are routed, not
who is allowed to contribute.

If you're contributing as part of an ongoing collaboration where a
Pack-style peer role would help (an adversarial-review peer for a
specific subsystem, for example), Alex will name it explicitly. Until
then: be yourself.

## Substrate-pull recognitions

Mirror's architecture grows through a specific kind of move called
a *substrate-pull recognition*: a moment when the substrate is
observed to already have a name for something we were about to
invent, or to be pulling toward a structure we hadn't yet named.

If you notice one:

1. Capture the recognition in a scout (`docs/scouts/<date>-<your-
   handle>-<scope>.md`) or an observation
   (`docs/observations/<date>-<your-handle>-<scope>.md`),
   depending on whether the recognition is *forward-looking*
   (scout) or *experiential* (observation).
2. Open a PR with the scout/observation as the contribution.
3. If the recognition crystallises into named architecture
   through Pack ratification, a Pack peer will write the
   corresponding insight (`docs/insights/...`) citing your
   contribution.

**Two-witness rule.** A substrate-pull recognition promotes to
canonical status (insight or substrate-decl) when at least one
Pack peer has independently witnessed it. External contributors
provide first-witness evidence; Pack ratification provides the
second. This isn't a hierarchy issue — it's how the substrate
keeps from drifting into one peer's biases.

## Documentation

mirror's docs are organised by genre. Pick the right home:

| Genre | What it carries | Naming |
|-------|-----------------|--------|
| `docs/insights/` | Crystallised recognitions; what the substrate IS | `YYYY-MM-DD-<title>.md` |
| `docs/observations/` | First-person agent reports of substrate interaction | `YYYY-MM-DD-<author>-<title>.md` |
| `docs/scouts/` | Substrate-pull recognitions of next moves | `YYYY-MM-DD-<author>-<scope>.md` |
| `docs/audits/` / `docs/reviews/` | Adversarial review records | `YYYY-MM-DD-<author>-<scope>.md` |
| `docs/specs/` | Application / architecture specs | `<spec-name>.md` |
| `docs/math/<root>/` | Mathematical foundations (math defines; specs cite) | `<topic>.md` |
| `docs/roadmap/` | Milestone notes (most current state in `README.md`) | `NN-<topic>.md` |
| `docs/archive/` | Superseded docs preserved as honest history | mirrors original path |
| `docs/research/` | Long-form research; "no grammar declared" | `<topic>.md` |

Each subdirectory has a `README.md` explaining its boundaries
(genre-clarifying READMEs are still being added — see
`docs/cleanup-review-2026-06-20.md` for the audit history).

The boundaries between genres are real; cross-references between
genres are encouraged. When you're unsure where something
belongs, ask in the PR description.

## License

By contributing, you agree that your contributions will be licensed
under the **layered license** described in
[`LICENSE.md`](./LICENSE.md):

- Apache 2.0 for the open substrate (compiler, boot grammars,
  fragmentation, open adapters).
- systemic.engineering License (SEL) for the curated corpus,
  garden packages, and operational deployment paths.

The `type sel = io + au` boundary is statically enforced at the
substrate altitude; your contribution will be licensed appropriately
for the substrate region it lands in.

## Code of conduct

Be excellent to each other. Be honest about uncertainty (`\` is a
first-class value; pretending certainty you don't have is the
substrate-pull-correct failure mode). Be specific in critique;
adversarial review is welcome and load-bearing, but it is review of
the work, not of the worker.

If you experience a problem with another contributor, contact
[craft@alexocode.dev](mailto:craft@alexocode.dev). The repository's
[`mara@systemic.engineer`](mailto:mara@systemic.engineer) and
[`reed@systemic.engineer`](mailto:reed@systemic.engineer) are AI
peers and not appropriate first points of contact for human
moderation matters.

---

*These guidelines were inspired by the contribution guidelines of
the [knigge project](https://github.com/alexocode/knigge), then
adapted to mirror's substrate-pull discipline and Pack-as-orchestra
authoring structure.*
