# The chain is the shatter

*2026-04-07*

A `.shatter` file is the chain of model invocations, not the encoded
result tree.

## The recognition

Until tonight, the working assumption was that a `.shatter` file is the
output of compilation: an encoded fragment tree on disk that some later
process reads back. That framing is wrong by one layer. The encoded tree
is what `mirror compile` writes today, but the encoded tree is just the
*current crystallization* of something else: the chain of model
invocations that produced it.

The `.mirror` file at convergence and the `.shatter` file are the same
artifact. The `.shatter` adds the optimal weights and the trajectory
beam — the record of *how* the form crystallized. Run the chain again
against the same input and you get the same crystal because the chain is
deterministic; the chain *is* the program.

> A `.shatter` is the chain of model invocations that, when refracted,
> reproduces the result. We don't compute. We crystallize.

## Vocabulary

Two operators carry the meaning. They are *not* the same operator viewed
differently — they are the sequential and parallel halves of the same
algebra.

- `|>` — **sequential composition.** Elixir-style forward pipe. The output
  of one model invocation becomes the input of the next. Cascade. The
  chain.
- `|` — **parallel superposition.** The quantum hold operator. Two (or N)
  branches both alive, neither collapsed. Also the variant separator in
  mirror grammar (`traversal verdict = pass | fail`). Also `split` in the
  Prism trait.

The same character `|` serves three roles, and under the new framing they
are *the same role*: variant declarations are literally superposition
declarations. A variant set is a held branch set. `split` walks the
branches without collapsing them. Crystallization happens in `refract`,
not in `split`.

This resolves a comment in `mirror_runtime.rs`. `Shatter::split` is
currently a conservative no-op marked TBD. The TBD is now resolved: `|`
*is* `split`. The implementation has not yet been updated to reflect this.
That is downstream work.

## What the editing experience becomes

If the chain is the program and the chain is editable, then editing a
`.mirror` file is no longer text editing. It is *continuous collapse*.

- Each `|` is a branch the AI is holding open.
- The AI formatter (`mirror fmt`, which today calls `mirror ai fate
  --train --out=<file>`) detects unnecessary holds and removes them.
- The file converges as you edit. Every save is one step further toward
  the crystal.
- A fully crystallized `.mirror` file has no `|` left except where the
  variant set is the actual semantics. The chain has collapsed into a
  cascade.

The compiler runs continuously; the editor is the inference loop.

## Cross-references

- `mirror/src/mirror_runtime.rs` — `Shatter`, `MirrorRuntime`,
  `CompiledShatter`, the Prism trait impl. The `split` and `zoom` TBD
  comments are the ones that get resolved by this insight.
- `coincidence::declaration` — `MirrorData`, `MirrorFragment`, `DeclKind`.
  The typed fragment that the chain crystallizes into.
- `fate::runtime::FateRuntime` — the inference primitive that runs at
  each step of the chain. `mirror ai <model> <file>` is one chain step.
- `mirror/src/main.rs` — `cmd_ai` is the chain stepper. The bash pipeline
  composing `mirror ai` invocations *is* the `.shatter` until the
  encoding catches up.
