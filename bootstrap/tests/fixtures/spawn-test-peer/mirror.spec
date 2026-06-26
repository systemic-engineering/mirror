in @mirror/cli
in @mirror/mosaic
in @mirror/spec
in @property
in @io

# Test fixture for Phase G v0 — empirical-path-traversal proof of
# `mirror spawn`. Used by bootstrap/tests/spawn.rs.
#
# The minimum mirror.spec a peer's home repo must carry for `mirror
# spawn <home>` to traverse the seven-piece composition per Mara's
# 2026-06-26 spawn-semantics insight (docs/insights/2026-06-26-spawn-
# is-substrate-leaving-ground-state.md). v0 reads project name + the
# pack{}.lead via simple text matching.

project test-peer {
  source ~d'shards/'

  pack {
    lead ~peer'~/.test-lead'
  }
}
