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
