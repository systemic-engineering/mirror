# mirror

## Fresh-session boot

1. Read `docs/loop/CURRENT.md` — the active arc state (landings, unapplied
   diffs, adjudication queue, blockers, immediate next actions).
2. Read `AGENTS.md` — Pack conventions, docs/math vs docs/specs discipline,
   cascade updates, SSH signing rules, --no-verify discipline.
3. If you're Reed: `~/.reed/00-NARRATIVE.md` through `~/.reed/04-TECH.md`
   boot sequence, then `~/.reed/tasks/pending/` and `~/.reed/tasks/important/`.

## Substrate discipline (load-bearing)

- **Substrate-honest is the mode. Always.** Two-paths framing ("here's
  honest / here's fast, I recommend fast") already breaks the mode. See
  memory `feedback-substrate-honest-is-the-mode` (Alex 2026-07-07).
- **Substrate-already-had-the-word.** Before inventing a family-root or
  species, grep. Landed instances this arc: `@cyberpunk`, `@magic`,
  tick-74 shatter spec, the slogan itself.
- **No Rust extension shortcuts.** Before authoring any `.rs` file, ask:
  can this be a shard body composing over @io? If yes: STOP. Do not
  write Rust. Reed's 2026-07-14 failure landed 5 substrate-dishonest
  Rust extensions bypassing FROZEN via the old marker. See Reed memory
  `feedback-no-rust-extension-shortcut` +
  `docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-during-gift-arc.md`.
  Marker renamed for `.rs` bypass: `[substrate-pull:realize]` →
  `[substrate-floor:@io-boundary]` (+ Seam gate: audit citation or
  `Signed-off-by: Seam` trailer).
- **Two-tick discipline** when collapsing: readable name over foundational.
- **SSH signing default.** NEVER override `gpg.format` or `user.signingkey`.
- **Sequential commits only.** `--no-verify` requires Alex in-transcript
  authorization OR pure-docs 📝 bypass (markdown-only).
- **Author attribution** per commit: `git -c user.name=<Name> -c
  user.email=<lowercase>@systemic.engineer commit -m ...`.

## Where to look

- `mirror.spec` — dogfood substrate root
- `shards/` — substrate declarations (family-roots, species)
- `shards/epistemologic/cybernetic/` — 13 landed cybernetic properties
- `bootstrap/src/lib.rs` — Rust bootstrap (transitional; substrate-pull
  collapse is the arc)
- `docs/loop/CURRENT.md` — active arc state
- `docs/math/YYYY-MM-DD-*.md` — mathematical foundations landed by session
- `docs/audits/YYYY-MM-DD-*.md` — Seam Phase D audits
- `docs/scouts/YYYY-MM-DD-*.md` — Taut drift scouts

## Pack peers

Reed / Mara / Seam / Taut / Glint. Coordination conventions in `AGENTS.md`.
Commit-as identities:
- `Reed <reed@systemic.engineer>` — orchestrator, RED-first tests, Seam-inline audits
- `Mara <mara@systemic.engineer>` — canonical spec author (math-first)
- `Seam <seam@systemic.engineer>` — adversarial review, Phase D audits
- `Taut <taut@systemic.engineer>` — grep-first drift scout, read-only
- `Glint <glint@systemic.engineer>` — essayist / prose cascade closure
