# Historical specs

These specs predate the collapse to the five operations
(`surface-simplification.md`, 2026-05-19). They describe architectures, surfaces,
and grammar shapes that no longer exist in the binary. They are kept here
because they explain *why* certain choices were made and what was tried before
the current shape — not because they describe the current shape.

When reading these:

- The Rust crate they reference is gone. The bootstrap is now `bootstrap/`.
- `@mirror/check`, `@mirror/build`, `@mirror/settle` (was: `@mirror/refract`), `@shatter` (as a verb),
  `@benchmark`, and `@run` were dissolved into `@craft`, `@kintsugi`, and
  `@beam`. See `../surface-simplification.md`.
- Concepts like "compiled CLI," the `.shatter` frontmatter format, and the
  pre-prism kintsugi pipeline are superseded by the in-tree grammars.

The canonical specs are in the parent directory. Start with `road-to-1.0.md`.

## 2026-06-12 spring-clean pass 2 — Mara

- `kintsugi-tick-1-results.md` (2026-05-12) — operational artifact: per-file
  OID log from the `reed/kintsugi-grammars` tick. Historical evidence of a
  single tick on a since-collapsed branch; not a spec. Only outside-this-dir
  citation was `road-to-1.0.md`'s historical-triage table, which already
  classified it as historical.

### Audit notes (no move; surfaced for the record)

Pass 2 scoped three supersession chains but found most candidates STILL
ACTIVELY CITED by current specs or shards (modified within the 06-10 cutoff
window). Per the move-don't-delete + cite-relationship-trumps-date rules,
those candidates stay in the active dir. The audit findings:

- **Kintsugi chain.** `kintsugi-shatter.md` is listed as CURRENT in
  `road-to-1.0.md` and cited by `kintsugi-ci-v0.1.md` + `bootstrap-retirement-plan.md`.
  `kintsugi-fracture-confidence-and-scene-dispatch.md` is cited by
  `kintsugi-ci-v0.1.md`, `mirror-grammar-self-hosted.md`,
  `gap-tension-tensor-substrate.md`. `kintsugi-minimum-runnable.md` is
  cited by `kintsugi-ci-v0.1.md`, `coincidence-hash-collapse.md`
  (2026-06-12), `spectral-hash-design.md`, `store-vs-db-and-the-cascade.md`.
  `kintsugi-wiring.md` is explicitly noted as the implementation guide for
  the wires under `kintsugi-formatter.md`'s mathematical lift; both citers
  flag "partially superseded" but use it. `trace-kintsugi-pipeline.md` is
  heavily cited from CURRENT shards (`shards/mirror/lens.mirror`,
  `shards/mirror/lens/refract.mirror`, `shards/mirror/spectral.mirror`).
  None safe to move.
- **Mirror runtime chain.** The brief's `mirror-runtime-evaluator.md` does
  not exist. `mirror-interpreter.md` is the interpreter-loop spec cited by
  `spectral-triple-binary.md` as the canonical reference for `\`-resolution.
  `mirror-runtime-gen-prism.md` is the substrate-altitude actor primitive
  explicitly distinguished from `@spectral`'s gen_prism in
  `spectral-runtime.md` ("Spectral's gen_prism uses this. Most things that
  want 'a process' only need this"); cited by `lsp-and-mcp.md` (3x),
  `scheduler-tower.md`, `kintsugi-wiring.md`, plus three live `boot/std/`
  grammars. These are altitude-distinguished, not superseded.
- **Eigenboard / eigensheaf.** Not a supersession. `eigensheaf.md` (2026-06-10)
  explicitly cites `eigenboard-representation.md` as the BASE bundle on
  which the sheaf-Laplacian eigenbasis sits ("the eigenboard's base is the
  five-operation graph"). Two altitudes: the bundle (eigenboard) vs the
  spectral decomposition over it (eigensheaf). Both stay; the boundary is
  documented inside `eigensheaf.md` §2.1.
