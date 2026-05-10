in @prism
in @meta
in @code/rust
in @git

-- mirror.spec: the mirror binary describes itself.
--
-- Two targets: boot (the grammar layer) and cargo (the Rust layer).
-- Kintsugi collapses them against each other.
-- The spec is the proof that they agree.

-- ── Targets ──────────────────────────────────────────────────────

type target = boot | cargo

-- boot: the grammar layer.
-- Everything in boot/ is a prism. The import graph is the build order.
prism @boot {
  focus  boot -> [grammar]
  split  grammar -> import_graph
  zoom   import_graph -> build_order
  refract compile(build_order) -> crystal
}

-- cargo: the Rust layer.
-- Five crates: prism, mirror, lens, spectral-db, spectral.
prism @cargo {
  focus  src -> [crate]
  split  crate -> dependency_graph
  zoom   dependency_graph -> build_order
  refract compile(build_order) -> artifact
}

-- ── The collapse ─────────────────────────────────────────────────
--
-- Boot and cargo must agree. Where they disagree, the seam is gold.
-- The loss measures drift between grammar and implementation.
--
-- When loss reaches zero, the Rust is generated from the grammar.
-- Until then, kintsugi holds them together.

zoom mirror(spec) -> imperfect {
  focus  spec
  split  spec -> [@boot, @cargo]
  zoom   collapse(@boot, @cargo) -> imperfect(mirror)
  refract settle
}

-- ── Properties ───────────────────────────────────────────────────

requires canonical_order
requires every_type_reachable
invariant deterministic
invariant pure
ensures always_halts

-- ── Build ────────────────────────────────────────────────────────

default(target) = native

out target
out @boot
out @cargo
out mirror
