# Taut scout — self-migration substrate-boundary (six targets)

**Date**: 2026-07-10 · **Author**: Taut <taut@systemic.engineer>
**Anchor**: Mara iter-17 (`beef270`), iter-18 (`129f618`), iter-19
(`78d5110`); Taut scout iter-10 (`8ac250e`); Reed's grounding grep.
Read-only.

## Executive summary

- **§1** LANDED. `shards/spectral.mirror` (5.1KB, `prism @spectral`)
  is namespace-parent-only per Loki §5 shrink (`17f0ee5`). NO
  instantiation edge to any spectral-triple. Instantiation is additive.
- **§2** GENUINE gap for `@knife` (2 prose hits, both in
  `reframe.mirror`). Substrate-already-had-the-word:
  **`@magic/distinction`** wins; secondary `sheaf_laplacian.λ₀`.
- **§3** Fracture machinery CONFIRMED (7+ species; uniform
  `resolve_*(opacity) -> morphism` with `content: splinter(ast)` at ONE
  file). Whole-shard migration is a SUPERSET; multi-file atomic settle
  NOT declared. New species needed.
- **§4** `spectral @foo` fault-plane LOCAL: 3-line delta at
  `bootstrap/src/lib.rs::collect_declared_namespaces` + optional parser
  arm. Two-tick clean (alias → (A,H,D)-witness).
- **§5** ZERO non-dangling `in @` consumers. ONE dangling consumer
  (`coherence-parametric.mirror:8` imports never-declared
  `@epistemologic/math/connes_spectral_triple`). Migration cost LOW.
- **§6** **β — LANDABLE WITH PREREQUISITES**. Min-cut: one new
  fracture species + one pact predicate + 3-line bootstrap delta →
  discharges spectral-triple move.

**Overall LRM**: **β** (localized, grep-audit-clean prereqs).

---

## §1 `shards/spectral.mirror` status + instantiation shape

Family-root exists as namespace-parent-only. Loki §5 shrink
(Seam-ratified `17f0ee5`, 2026-07-01) collapsed it from a
BEAM-on-mirror family to a path-container. Body:

```
prism @spectral { focus spectral  project spectral  split spectral
                  shift spectral  settle spectral }
```

NO `instantiates @<foo>` edge exists today. Alex's proposal
(`@spectral INSTANTIATES @epistemologic/spectral_triple`) is a NEW
substrate-decl — additive, not a rewrite. Nine species under
`shards/spectral/` (root, parent, portal, registry, entanglement,
gen_prism, supervisor, restart_intensity) also carry no spectral-triple
edge.

## §2 @knife gap + substrate-already-had-the-word

Two prose hits only, both in
`shards/epistemologic/cybernetic/reframe.mirror` (lines 12, 15, 58 —
all forward-promise refs to `@knife.cut(state_space_K)`). No
`shards/knife.mirror`, no family root, no species. Genuine gap.

**Candidates for the load-bearing filter operation**:

1. **`@magic/distinction`** (`shards/magic/distinction.mirror`, 7.7KB,
   `prism @magic/distinction`) — Spencer-Brown mark. Strongest match:
   the substrate's existing "cut IS distinction" carrier.
2. **`@epistemologic/math/sheaf_laplacian.lambda_zero`** — Fiedler
   algebraic-connectivity floor; substrate's quantitative
   spectral-gap filter.
3. **`@epistemologic/pact/*`** — declarative predicates already carry
   the "which candidates admit" surface (substrate_source_in_shards,
   path_matches_namespace, parent_acyclic).

Recommendation: **do NOT land `@knife` as new family-root.** If a cut
carrier is genuinely needed beyond distinction's expressivity, land it
as `@magic/distinction/cut` (depth-2 glass) — substrate-already-had-
the-word.

## §3 Kintsugi AST-rewrite machinery capability

Uniform shape verified across `gate.mirror`, `keyword.mirror`,
`symbol_lift.mirror`, `partials_align.mirror` (all
`glass @kintsugi/fracture/<name>`):

```
resolve_<name>(opacity: opacity) -> morphism {
  morphism {
    content: splinter(ast) { content: opacity.location.file,
                             ast: @meta/ast, transparency: success },
    score: dissonance { roughness: opacity.property, partials: 1 },
    expected: authentic,
  }
}
```

One opacity → one morphism → one AST-node rewrite at ONE file
(`opacity.location.file`). `partials: 1` is explicit throughout.

**Whole-shard migration is a SUPERSET**:
- File-move needs TWO location refs (source + target); splinter carrier
  is single-location.
- Import-update across N consumers needs N morphisms OR one morphism
  carrying multi-file settle — not a declared shape today.
- Oscillate loop's `active_pass` iterates opacities one-by-one; no
  transactional-atomic multi-morphism settle.

**Verdict**: current fracture surface expresses single-site rewrites
only. Path forward: declare `@kintsugi/fracture/relocate` widening
`splinter(ast)` to accept `(source_ref, target_ref, [import_rewrite])`.
This is the cleanest substrate-pull; the leaky alternative
(transactional N-morphism settle) is worse.

## §4 `spectral @foo` grammar-admittance fault-plane

Two admission surfaces:

1. **Grammar parser**: `bootstrap/src/tokenize.rs` +
   `bootstrap/src/lib.rs` walker over `AstKind::Focus`. If
   `spectral @foo` is prism-alias in tick A, the parser MAY need
   zero delta (verify at parse-time); (A,H,D) discrimination lands in
   tick B.

2. **Corpus-scanner** (`bootstrap/src/lib.rs::collect_declared_namespaces`,
   ~line 2100). Hard-coded prefixes:

   ```
   line.strip_prefix("glass ") / "prism " / "grammar "
   ```

   Extending to `"spectral "` = 3-line delta. Without this, `spectral
   @foo` decls are INVISIBLE to the semantic-import resolver;
   consumers surface as unresolved (dark_count += 1).

**Grep for existing collisions**: no `spectral \w+` line-start patterns
outside path-refs (`@spectral/...`). Clean fault-plane.

**Two-tick discipline available**:
- **Tick A**: prism-alias. 3-line bootstrap delta. `spectral @foo`
  compiles identically to `prism @foo`. (A,H,D) IMPLICIT.
- **Tick B**: `@epistemologic/pact/is_spectral_triple` gate enforces
  `type algebra`, `type hilbert_space`, `type dirac_op` on every
  `spectral @foo`. Fault-plane grep-clean at landing.

## §5 spectral-triple migration cost + consumer inventory

**Direct `in @` consumers** (grep):
- `in @epistemologic/math/spectral-triple` — **0 matches** (declared
  name in `boot/std/...`)
- `in @epistemologic/math/spectral_triple` — 0 matches
- `in @epistemologic/spectral_triple` — 0 matches
- `in @epistemologic/math/connes_spectral_triple` — **1 match** at
  `shards/epistemologic/cybernetic/coherence-parametric.mirror:8`
  (DANGLING — no substrate-decl declares this name).

**Doc-string / backlink refs** (do NOT block migration):
- 40+ shards mention `spectral-triple.mirror` in prose;
- 20+ carry `[[architecture-connes-spectral-triple]]` backlinks;
- doc-string cascade is a follow-up tick, not a prereq.

**Fault-plane on migration** (`boot/std/.../spectral-triple.mirror` →
`shards/epistemologic/spectral_triple.mirror`):
- rename hyphen → underscore per shard convention;
- drop `/math/` sub-namespace;
- rewrite the file's own `grammar @<path>` + `out @<path>` lines;
- rewrite the ONE dangling consumer (or land it as
  substrate-declared and the consumer is correct-by-birth).

**Foerster-eigenform witness**: the family-root that TYPES spectral
triples must itself type-check as one. Grammar today declares
`type algebra / hilbert_space / dirac_op / spectral_triple` — surfaces
the (A,H,D) at type-altitude. Whether the DECL itself is a triple
instance is tick-B territory; NOT required for the file move.

**Migration cost**: **LOW**. 1 dangling `in @` consumer, 1 self-ref,
0 non-dangling consumers.

## §6 Self-migration loop LRM + minimum-cut

**LRM**: **β** (LANDABLE WITH PREREQUISITES).

**Minimum-cut — smallest tick discharging one real migration**:

1. **PREREQ-1** (bootstrap): 3-line delta to
   `collect_declared_namespaces` admitting `"spectral "` (Reed's
   territory).
2. **PREREQ-2** (Mara canonical): declare
   `@kintsugi/fracture/relocate.mirror` — new fracture species widening
   splinter to `(source_ref, target_ref, [import_rewrite])`.
   Substrate-pull-clean; oscillate's `active_pass` reads it identically
   to existing species.
3. **PREREQ-3** (Mara): declare
   `@epistemologic/pact/is_load_bearing_in_std` — predicate whose
   verdict identifies boot/std decls with real
   `in @<path>` consumers (grep-first check).
4. **Discharge**: migrate `boot/std/epistemologic/math/spectral-triple.mirror`
   → `shards/epistemologic/spectral_triple.mirror` via
   `@kintsugi/fracture/relocate` triggered on PREREQ-3's verdict. The
   dangling consumer at `coherence-parametric.mirror:8` is rewritten
   atomically by the same morphism's import_rewrite set.
5. **Witness**: the migration morphism is itself a well-typed instance
   of `@kintsugi/fracture/relocate` — the loop closes on its own math.

After minimum-cut, subsequent boot/std migrations are consumer-pull:
same pact predicate + relocate fracture, no new substrate-decl.

**Explicitly NOT in min-cut**: `@knife` family-root (§2),
`spectral @foo` as new head (§4 tick B), `@spectral INSTANTIATES`
edge (§1 additive), doc-string cascade (40+ backlinks; follow-up).

## Recommended next action for Reed

Land **PREREQ-2** first (Mara canonical author):
`@kintsugi/fracture/relocate` substrate-decl. Highest-leverage single
shard — unlocks the self-migration loop while staying substrate-
honest at fracture-body altitude. PREREQ-1 (3-line bootstrap) and
PREREQ-3 (pact predicate) fold into the same session. The
spectral-triple discharge closes the loop; the build system's math IS
then the migration engine — for every subsequent boot/std file.
