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
