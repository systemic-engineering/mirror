# kintsugi-tournament — Fate-resolved merge as morphism on the eigenboard sheaf

*2026-05-22. Reed. Spec.*

Status: **Red** (the collision is concrete — `runtime.mirror` x2 and
`nl.mirror` x2 in Tick 4b.3's `kintsugi --transform='grammar => glass'
--out=mirror/` pass; the resolution is currently "Alex decides by
hand"; this spec closes the gap with a Fate tournament whose outcome
is a typed bundle automorphism on the eigenboard sheaf.)

Depends on:
- `mirror/docs/specs/kintsugi-formatter.md` — the contraction map; the
  five iteration stages; the Banach convergence guarantee. The
  tournament IS stage 1 (propose) + stage 3 (elect) for a specific
  family of holes: collisions in the migrate target.
- `mirror/docs/specs/eigenboard-representation.md` — the principal
  G-bundle on the five-operation graph; a tournament outcome is a
  bundle automorphism whose connection-form change records the
  resolution.
- `mirror/docs/specs/parser-as-prism-grammar.md` — FP1/FP2/FP3 fixed
  points; the post-merge state must continue to satisfy them.
- `mirror/docs/specs/kintsugi-wiring.md` — the eight wires; wire 2
  (five-model fan-out) and wire 3 (conductivity contest) are exactly
  the tournament's proposal and elimination steps.
- `mirror/docs/specs/au-and-conductivity.md` — au IS Fate's output
  type; conductivity IS the scoring function's primary axis.
- `mirror/boot/std/fate/tournament.mirror` — the tournament rule
  grammar (greedy / beam / elite / halving / tabu / anneal / ucb)
  already declared; this spec is the *first concrete consumer*.
- `mirror/boot/std/ai/{abyss,introject,cartographer,explorer,fate}.mirror`
  — the five candidate proposers.
- `spectral/docs/specs/kintsugi-collapse.md` — the structural-diff
  collapse primitive; the merge-strategy vocabulary builds on it.
- `spectral/docs/specs/mirror-kintsugi.md` — the Fate-driven
  simplification pipeline; the original tournament-as-grammar test
  fixture.
- Reed memory: `project-eigenboard-is-sheaf` (cellular sheaf on the
  five-operation graph; restriction maps = conductivity tensor;
  Reflection writes morphisms) and `project-au-conductivity`
  (Fate's output type; gold-as-conductor; tournament as conductivity
  contest).
- `mirror/bootstrap/src/main.rs::cmd_kintsugi_migrate` — the migrate
  loop where the collision is detected today.
- `mirror/bootstrap/src/pipeline.rs::execute_pipeline` — the mq-query
  surface kintsugi rides; the rewrite rule is parsed here.

Unblocks:
- The Tick 4b.3 collision (`runtime.mirror` x2, `nl.mirror` x2)
  resolves automatically; Alex's hand stops being load-bearing.
- The migrate command lands a `--resolve=<mode>` flag with
  `tournament` as one option (`fail` stays the default).
- The structural-collision case generalises to *any* under-determined
  morphism in the eigenboard sheaf: every kintsugi choice point is a
  tournament-shaped hole. The same code path resolves naming
  collisions, near-identical declarations, and inferred grammar
  extensions.
- Reflection's gestalt entry for a kintsugi tick gains a typed slot
  for the tournament outcome (which strategy won, what was eliminated,
  what the holonomy delta was).
- The first observable end-to-end Fate tournament in the mirror
  bootstrap: not a synthetic test fixture, but a real migrate-time
  decision the system makes about its own source tree.

---

## 0. Thesis

**A collision is an under-determined morphism in the eigenboard
sheaf.** The kintsugi formatter's stage 1 (propose) sees the
ambiguity; the Fate tournament resolves it; the winning strategy IS a
bundle automorphism whose holonomy strictly decreases. The outcome is
written to the eigenboard's section history as a typed gestalt entry —
a morphism that Reflection can observe, replay, and learn from.

The collision in Tick 4b.3 is one instance of a family. Today the
family has size two:

- `boot/std/nl.mirror` (161 B, terse interface declaration) vs
  `boot/std/mirror/nl.mirror` (3.2 KB, full `@nl` grammar)
- `boot/std/runtime.mirror` (169 B, declaring template/io/action) vs
  `boot/std/mirror/runtime.mirror` (768 B, declaring the
  resolve→reflect→interpret pipeline)

Both pairs collapse to the same destination basename when
`cmd_kintsugi_migrate` strips `std/mirror/` and `std/`. The current
behaviour is to error and stop. The desired behaviour is to:

1. Detect the structural collision.
2. Enumerate candidate merge strategies (closed vocabulary).
3. Ask Fate to score each strategy under the eigenboard's connection.
4. Elect the lowest-holonomy strategy by tournament.
5. Apply the winning strategy as a kintsugi tick.
6. Continue the settle.

Every step is a Prism on the eigenboard bundle. The whole pipeline is
one `apply_h(@kintsugi/tournament, collision)` call. The math is the
formatter's contraction-map argument applied at the granularity of
file-system identity rather than AST identity.

---

## 1. The shape of a tournament round

A tournament round is **detect → enumerate → score → eliminate →
apply**. Diagrammatically:

```
        kintsugi settle reaches a choice point
                       │
                       ▼
              ┌────────────────────┐
              │  detect collision  │  (structural query on the sheaf)
              └─────────┬──────────┘
                        │
                        ▼
              ┌────────────────────┐
              │  enumerate cands   │  (closed merge-strategy vocab)
              └─────────┬──────────┘
                        │
                        ▼
              ┌────────────────────┐
              │  score (Fate)      │  (five models propose κ;
              └─────────┬──────────┘   tournament reduces to ranking)
                        │
                        ▼
              ┌────────────────────┐
              │  eliminate         │  (elite(1).beam(k).halving(η))
              └─────────┬──────────┘
                        │
                        ▼
              ┌────────────────────┐
              │  apply winner      │  (rewrite the sheaf section)
              └─────────┬──────────┘
                        │
                        ▼
        kintsugi settle continues with new section
```

The loop terminates when the section reaches a Lawvere fixed point
(per `kintsugi-formatter.md`'s stage 5). If the winner's holonomy
exceeds the convergence threshold ε, the round aborts to failure
handling (per the formatter's three failure stages).

### 1.1 Collision, defined structurally

A collision is not "two files share a basename." A collision is **an
under-determined morphism in the cellular sheaf on the operation
graph.**

Formally: let `F` be the eigenboard sheaf (per
`project-eigenboard-is-sheaf`). A morphism `φ: F → F'` is a section
rewrite; the sheaf restriction maps determine which rewrites are
admissible. The morphism is *determined* if exactly one admissible
`φ` exists; *under-determined* if zero or more than one exists. The
formatter resolves the under-determined case.

File-system collisions are one shape of under-determined morphism:
the restriction map at the migrate edge (`std/mirror/X.mirror →
mirror/X.mirror`) takes two distinct sources to the same target. The
diagram does not commute uniquely. The sheaf does not know which
source to keep.

Other shapes of under-determined morphism in the same family:

| Shape | Where it surfaces | Today's resolution |
|-------|-------------------|--------------------|
| Two files share a basename after path canonicalisation | `cmd_kintsugi_migrate` | error / Alex |
| Two `grammar @<x>` declarations in different files claim the same namespace | resolve pass | last-wins |
| Two `type X = …` definitions in the same compilation unit diverge | parser | error |
| `in @a/b` and `in @a/c` both export a name `f` | name resolution | first-wins |
| Two property assertions on the same io binding disagree | model checker | error |
| Two au candidates from different Fate models tie on κ | tournament | beam(k) carries both |
| A grammar extension is inferred by two different Reflection paths | `@cogito.strategy` | last-wins |

The tournament's job is **uniform**: each shape is one tournament
round over a strategy vocabulary specific to that shape. The current
spec specifies the file-system case in detail and sketches the
generalisation.

### 1.2 Detection

Detection is a structural query on the sheaf, written as an mq query
(per `mirror/boot/std/code/mq.mirror`):

```
sheaf > restriction_map[input=$source]
      :where(target == restriction_map[input=$other_source].target)
      :where($source != $other_source)
```

In the file-system case, the restriction map IS the path
canonicalisation function:

```
strip_prefix("std/mirror/")
  or strip_prefix("std/")
  then apply_basename_rewrites(rules)
```

A collision is two distinct inputs that produce the same canonical
output. The detector walks the migrate's file list, computes the
canonical path for each, and groups by target. Any group of size > 1
IS a collision.

Detection is `O(n)` over the file list with one canonicalisation per
file. The result is a list of `Collision { target, sources: [Path] }`.

---

## 2. The merge-strategy vocabulary

A merge strategy is a **structural rewrite that, applied, removes the
collision.** Each strategy is a Prism: input is the collision
(`{target, sources}`), output is an `imperfect(plan, loss)` where the
plan is a sequence of file operations that takes the source tree to a
collision-free state.

### 2.1 The vocabulary, declared as grammar

```mirror
# mirror/kintsugi/merge.mirror (new — Tick 4b.3.5)
in @prism
in @kintsugi
in @io
in @ai/fate
in @epistemologic

grammar @kintsugi/merge {
  # the collision: two distinct sources mapping to the same target
  type collision = {
    target:  path,
    sources: [path],
  }

  # a strategy plan: the operations that, applied, remove the collision
  type plan = {
    strategy: strategy,
    ops:      [file_op],
    losses:   [au],   # per-criterion loss (see §3 scoring)
  }

  type file_op =
    | write(path, bytes)
    | delete(path)
    | rename(path, path)
    | merge_into(path, [path])      # write a synthesised union

  # the strategy vocabulary — CLOSED sum type (one variant per named
  # strategy), expressed in mirror, not in Rust. open extension
  # requires a kintsugi pass over this grammar itself; this is
  # deliberate (see §2.3).
  type strategy =
    | keep_both(path, path)
      # rename one source so both survive at distinct targets.
      # e.g. boot/std/nl.mirror → mirror/nl/interface.mirror,
      #      boot/std/mirror/nl.mirror → mirror/nl.mirror
    | delete_smaller(path)
      # drop the source whose AST is a structural subset of the
      # other. only applicable when one source's content is dominated
      # by the other.
    | inline(path, path)
      # splice the smaller into the larger as an additional grammar
      # action. only applicable when both sources share a namespace
      # AND their action sets are disjoint.
    | union(path, path, path)
      # write a third path whose content is the AST-level union of
      # both sources. requires no naming collisions in the union;
      # falls back to keep_both if union is ill-defined.
    | disambiguate_via_namespace(path)
      # the source declares `grammar @x/y`; rewrite the migrate
      # target to honour the declared namespace rather than the
      # path-derived one. e.g. boot/std/mirror/nl.mirror declares
      # `grammar @nl("nl",...)` — disambiguate writes to mirror/nl.mirror.
    | supersede(path)
      # one source is structurally newer and the other is a stub.
      # delete the stub; keep the rich source. detected by AST node
      # count + content-OID divergence + git history.

  # ENUMERATE: produce all applicable strategies for a collision.
  enumerate(collision) -> [strategy] { \ }

  # APPLY: realise a strategy as a plan (file_ops + losses).
  apply(collision, strategy) -> imperfect(plan, loss) { \ }

  # the public-facing tournament entry. delegates to
  # @fate.infer with `config: tournament_bracket(...)` (§4).
  resolve(collision) -> imperfect(plan, no_winner, loss) { \ }
}

out collision
out plan
out strategy
out file_op
out enumerate
out apply
out resolve
```

### 2.2 The six strategies, walked

**`keep_both(a, b)`.** Apply a disambiguating rename to one of the
sources before path canonicalisation runs. Concretely: the source
with the *less specific* declared grammar tag gets a path suffix.
For the `nl.mirror` collision, `boot/std/nl.mirror` is the stub
interface; `boot/std/mirror/nl.mirror` declares the rich `@nl`
grammar with bare/markdown/code-inline forms. `keep_both` writes:

- `mirror/nl/interface.mirror` ← `boot/std/nl.mirror`
- `mirror/nl.mirror`            ← `boot/std/mirror/nl.mirror`

Loss surface: small. The renaming preserves both grammars; the
imports that reference `@nl` must be checked (this is namespace
integrity, scored in §3).

**`delete_smaller(p)`.** Detect that one source's AST is dominated
by the other's (subset of declarations, no novel actions, no
distinct properties). Delete it. For `runtime.mirror`:
`boot/std/runtime.mirror` declares an abstract `@runtime` grammar
with three templates; `boot/std/mirror/runtime.mirror` declares
`@mirror/runtime` with two concrete actions. They are *not*
dominance-comparable — neither subsumes the other. `delete_smaller`
does not apply.

Loss surface: zero on dominance; ∞ when not applicable (filtered out
in enumeration).

**`inline(into, from)`.** When both sources share the *same*
namespace (declared `grammar @x` matches), splice the smaller's
actions into the larger and delete the smaller. The two `nl.mirror`
files declare different namespaces (`@nl` vs `@nl("nl", …)` with
aliases) — `inline` requires care: the alias declaration in the
richly-typed one absorbs the stub's `type nl(text)` and `#(nl)`
declarations.

Loss surface: medium. Cross-reference integrity may break if the
stub's `in @nl` referrers expect the simpler shape.

**`union(a, b, new_path)`.** Synthesise a third file whose AST is
the sheaf-union of the two sources (per `kintsugi-collapse.md`
§2's `merge_grammars` function). Delete both originals. The new
path is chosen by the tournament (typically the canonical target).

Loss surface: medium. Carries divergence as `Imperfect::Partial`
loss bits per `kintsugi-collapse.md`'s structural-distance metric.

**`disambiguate_via_namespace(file)`.** Read `grammar @<x>` from the
file; rewrite the migrate target to match. For
`boot/std/mirror/nl.mirror` declaring `grammar @nl("nl",
"natural-language", "bare")`, the disambiguator picks the
canonical name `nl` and writes `mirror/nl.mirror`. The other source
(`boot/std/nl.mirror`, declaring `grammar @nl`) ALSO claims `@nl`;
this strategy alone does not break the tie. Combined with
`delete_smaller` or `inline` it does.

Loss surface: zero when one source has a strictly more specific
grammar tag; ambiguous otherwise.

**`supersede(p)`.** Detect via git history that one source is the
active evolution of the other (one is older, has been refactored
into the other, is not imported anywhere else). Delete the
superseded one. For `runtime.mirror`, the file `boot/std/runtime.mirror`
is dated 2026-05-19 17:04 (pre-staircase); `boot/std/mirror/runtime.mirror`
is dated 2026-05-20 22:21 (post-staircase, declares the actual
compilation pipeline). The newer file is the active one;
`supersede` proposes deleting the older.

Loss surface: small when import-graph confirms the superseded file
is dead; ∞ when it's still referenced.

### 2.3 Closed vs open vocabulary — decision

**Decision: closed sum type, six variants, today.** The vocabulary
is declared as `type strategy = | keep_both | delete_smaller | …`
in `mirror/kintsugi/merge.mirror` — a closed Choice in the mirror
grammar, not a Rust enum. The closure is grammatical: extending the
variant set is a kintsugi pass over the merge grammar itself, not a
Rust recompile. Justification:

1. **The scoring function (§3) must be uniform across strategies.**
   A closed sum type lets the scoring function dispatch per
   variant and use strategy-specific loss components. An open
   combinator vocabulary requires every new strategy to teach the
   scorer about itself, which violates the formatter's
   contraction-map argument (the contraction factor `γ` would
   depend on the unboundedly-many open strategies).

2. **Closed sum types are content-addressable.** A `Combinator::Choice`
   over the strategies hashes to one OID; the bootstrap can verify
   the strategy set with one comparison. Open vocabularies would
   require recursive hashing of arbitrary user-supplied strategy
   trees.

3. **The grammar is sub-Turing.** Per `mirror-compile-bootstrap.md`,
   all of mirror's bodies must be sub-Turing decidable. Open
   combinator vocabularies admit fixed-points whose decidability is
   not guaranteed; closed sum types always terminate (the variant
   set is finite).

4. **Open extension is available via a kintsugi pass on the merge
   grammar itself.** Adding a seventh strategy is a one-line edit to
   `mirror/kintsugi/merge.mirror` followed by a kintsugi settle.
   The merge grammar is itself a `.mirror` file; structural changes
   to it go through the same loop that resolves any other migration
   — exactly the kintsugi-formatter property one level up. This is
   *correct*: a new strategy IS a structural change to the merge
   vocabulary, and structural changes go through kintsugi.

The vocabulary is closed *at this kintsugi tick*. The next tick can
extend it (per `kintsugi-formatter.md`'s autopoietic-closure
property). The current six cover every collision pattern surfaced
in the boot tree audit.

> **Terminology note (2026-05-22).** Earlier drafts said "closed
> enum." That phrase was Rust-flavoured shorthand. In mirror's own
> language the strategy set is a **closed sum type** declared as
> `type strategy = | … | …` and compiled into a `Combinator::Choice`
> over the named variants. The closure is grammatical, not
> implementation-level; the strategy vocabulary lives in
> `mirror/kintsugi/merge.mirror`, not in Rust. Any remaining
> appearances of "enum" in this document refer to that sum type,
> not to a Rust language construct.

### 2.4 Strategy applicability — enumeration

`enumerate(collision) -> [strategy]` filters the six variants to
those applicable to the given collision. The applicability matrix:

| Strategy | Applies when |
|----------|--------------|
| `keep_both` | Always. Universal fallback. |
| `delete_smaller` | One source's AST is a structural subset of the other (dominance check). |
| `inline` | Both sources declare the *same* namespace AND their action sets are disjoint. |
| `union` | The structural diff is `NearIdentical` (per kintsugi-collapse). |
| `disambiguate_via_namespace` | At least one source declares a `grammar @<x>` that uniquely determines a path different from the colliding one. |
| `supersede` | Git history shows one source is the active evolution of the other AND the superseded one is not imported elsewhere. |

For the two Tick 4b.3 collisions:

- **`nl.mirror`:** `keep_both`, `disambiguate_via_namespace`, `inline`
  (the namespaces differ but the rich one declares aliases including
  the stub's name).
- **`runtime.mirror`:** `keep_both`, `supersede` (newer file is the
  active one; older declares an abstract grammar that no resolved
  import uses post-staircase).

The tournament scores the applicable set; the closed enum's filter
is cheap; the winner is the strategy with lowest κ.

---

## 3. The scoring function

Multi-criterion. Each criterion is a structural query on the
post-merge sheaf. The scoring function returns a vector
`κ ∈ ℝ⁵` whose components measure five different conductivity
dimensions (one per gutter-lens duality, per
`spectral/docs/specs/gutter-lenses.md`).

### 3.1 The five criteria

**C1. Kintsugi fixed-point reachability (MANDATORY).** Does the
post-merge state satisfy `tokenize ∘ render = id`? Concretely: after
applying the strategy's plan, can the migrated file tree be
tokenized and rendered back to byte-identical form?

Query (mq):
```
post_merge > files > apply(tokenize) > apply(render) :equals(post_merge.files.content)
```

Result: boolean. A `false` here makes the strategy ineligible
(infinite κ on this axis; tournament eliminates immediately).

**C2. Namespace integrity.** No `grammar @<x>` declaration is lost
or altered without being explicitly replaced. The set of declared
namespaces in the post-merge state must be a superset of the union
of declared namespaces in the pre-merge state, modulo strategy-
declared deletions.

Query (mq):
```
pre_merge > grammar_decls[name=$n] - strategy.deletions.namespaces[name=$n]
  :is_subset_of(post_merge > grammar_decls > [name])
```

Result: integer (count of dropped namespaces). Zero is optimal;
strictly positive contributes proportionally to κ.

**C3. Cross-reference integrity.** No `in @<x/y>` import resolves to
a missing target after the merge. Every `in` statement in the
post-merge tree must point to an existing grammar.

Query (mq):
```
post_merge > imports > [ref] :all(target_exists)
```

Result: integer (count of dangling references). Zero is optimal.

**C4. OID churn (minimal-delta tiebreaker).** Among strategies that
pass C1–C3, prefer the one with the smallest structural delta —
fewest content-OIDs changed in the post-merge state vs the
pre-merge state. Combinator-tree OIDs are compared per file; the
delta is the count of files whose OID changed.

Query (mq):
```
pre_merge.files :zip(post_merge.files)
  :filter(fst.content_oid != snd.content_oid)
  :count
```

Result: integer (count of OID changes attributable to the strategy).
Lower is better.

**C5. Holonomy.** The cycle-averaged holonomy (Magnot 2025) of the
post-merge section transported around the kintsugi loop. This is the
formatter's stage 2 measurement applied at the strategy level.
Query (mq, via @hash/coincidence):
```
post_merge > eigenboard > section > holonomy
```

Result: f64 ∈ [0, 1]. Lower is better; zero is the closure level.

### 3.2 Composition: pareto front, then lexicographic

The five components are not commensurable. We do not combine them
with a weighted sum because the weights would be undecidable (each
collision has a different geometric flavour). We compose them
**lexicographically by tier:**

```
tier 1 (gates):   C1 (kintsugi reachability)
                  must be true; false eliminates immediately.
tier 2 (gates):   C2 (namespace integrity)
                  must be zero; nonzero eliminates immediately.
tier 3 (gates):   C3 (xref integrity)
                  must be zero; nonzero eliminates immediately.
tier 4 (rank):    C5 (holonomy)
                  sort ascending. lowest holonomy wins.
tier 5 (tiebreak): C4 (OID churn)
                  sort ascending. fewest changes wins.
```

Tiers 1–3 are GATES (boolean elimination). Tiers 4–5 are RANK
(lexicographic ordering). Ties at tier 4 (within `ε_holonomy`) break
at tier 5 (OID churn). Ties at tier 5 break randomly with a seed
derived from the collision's content-OID (deterministic; replayable).

**Why not pareto front?** A pareto front is the right shape when all
components are commensurable axes of value. Here, three are gates
(boolean) and two are ranks (continuous). The gate/rank distinction
maps cleanly onto lexicographic composition, and lex composition has
a unique winner up to the deterministic seed. A pareto front would
leave ties unresolved and require an additional selection rule.

**Why not weighted sum?** The weights would be load-bearing
parameters with no principled derivation. Lex composition has zero
free parameters at the tier boundary (gates are absolute; ranks are
ordered).

### 3.3 Scoring as mq-query

The scorer is a single composed mq pipeline:

```
@kintsugi/merge.apply(collision, strategy)
  |> @epistemologic/property/kintsugi_reachable
  |> @epistemologic/property/namespace_integrity
  |> @epistemologic/property/xref_integrity
  |> @hash/coincidence.holonomy
  |> @epistemologic/property/oid_churn
  |> @kintsugi/merge.compose_lex
```

Each stage is a Prism over the merged state. `compose_lex` is the
lexicographic composer; its body is total (closed enum of tier
orderings). The whole pipeline is sub-Turing decidable per the
formatter's stage-2 decidability proof.

### 3.4 Worked examples

**`nl.mirror` collision:**

| Strategy | C1 | C2 | C3 | C5 (κ) | C4 |
|----------|----|----|----|-----|----|
| `keep_both` | ✓ | 0 | 0 | 0.18 | 2 |
| `disambiguate_via_namespace` | ✓ | 0 | 0 | 0.05 | 1 |
| `inline` | ✓ | 1 (`@nl` stub absorbed) | 0 | 0.12 | 2 |

Gate C2 eliminates `inline` (a namespace is lost without an explicit
deletion). `disambiguate_via_namespace` wins on C5 (lowest holonomy:
the rich grammar's declared aliases already cover the stub's surface;
the merge is geometrically trivial).

**`runtime.mirror` collision:**

| Strategy | C1 | C2 | C3 | C5 (κ) | C4 |
|----------|----|----|----|-----|----|
| `keep_both` | ✓ | 0 | 0 | 0.22 | 2 |
| `supersede(boot/std/runtime.mirror)` | ✓ | 1 (drops `@runtime`) | check | 0.07 | 1 |

Gate C2 fails for `supersede` IF anything still imports `@runtime`.
The import-graph check is part of C2's structural query. For the
current boot tree, no resolved `in @runtime` exists (the abstract
`@runtime` was historical, pre-staircase). C2 passes;
`supersede` wins on C5.

The winning strategies match what Alex would have chosen by hand.
This is the spec's correctness criterion (per §8 acceptance).

---

## 4. Fate's role

### 4.1 What Fate provides today

Reading `mirror/boot/std/fate/tournament.mirror`:

```mirror
grammar @fate/tournament {
  type rule = greedy | beam(u64) | elite(u64) | halving(u64)
            | tabu(u64) | anneal(f64) | ucb(f64)

  tournament(rules, [hole]) -> [resolution] { \ }
  candidates(hole) -> [resolution] {
    @ai/abyss.depth(hole)
    @ai/introject.pattern(hole)
    @ai/cartographer.map(hole)
    @ai/explorer.explore(hole)
    @ai/fate.select(hole)
  }
  compose(rule, rule) -> rule { \ }
}
```

The `tournament(rules, holes)` action accepts a composed rule (e.g.
`elite(1).beam(8).halving(3)`) and a list of holes; returns a list of
resolutions. Body is `\` — Fate's tournament is declared as grammar
but not yet implemented. The MCP wrapper script
(`bin/mirror-mcp`) advertises `mirror_fate` but the subcommand does
not exist in the binary (per `road-to-1.0.md` Tick 4).

> **Reframe (2026-05-22).** Earlier drafts spoke of
> `@fate.tournament.tournament` as a top-level call — the doubled
> `tournament` reflected that today's grammar declares a `tournament`
> action inside an `@fate/tournament` grammar. Alex's call: **Fate
> has one public surface, `@fate.infer`.** Tournament is a
> *configuration* of inference, not a separate action. The bracket
> rule (`elite(1).beam(8).halving(3)`) is a value of type
> `tournament_bracket` passed in the `config` slot of
> `@fate.infer(query, config)`. The current `@fate/tournament`
> grammar retires; its `rule` sum type migrates into
> `@fate.tournament_bracket`. Beam, halving, elite, etc., become
> values constructing a `tournament_bracket`, not separate actions.
>
> The unified surface:
>
> ```mirror
> grammar @fate {
>   type config =
>     | single_shot
>     | tournament_bracket(rule)
>     | …                          # future strategies, same place
>
>   type rule = greedy | beam(u64) | elite(u64) | halving(u64)
>             | tabu(u64) | anneal(f64) | ucb(f64)
>
>   compose(rule, rule) -> rule { \ }
>
>   infer(hole_oid, context, config) -> imperfect([candidate], no_proposal, loss) { \ }
> }
> ```
>
> Everywhere the rest of this spec says
> `@fate.tournament.tournament`, read `@fate.infer(…, config:
> tournament_bracket(…))`. The doubled-name was a transient
> artefact of the staircase; the unified surface is what lands.

The wiring needed for tournament merges:

1. **A `mirror_fate` MCP tool implementation** (or equivalently a
   `mirror fate` subcommand) that takes a `collision` (serialised
   as JSON or a path-tuple) and returns a `[strategy]` (the
   tournament-ordered candidate list). Today this tool is advertised
   but unwired.

2. **A way to feed structural data to Fate's five models.** The five
   models (Abyss/Introject/Cartographer/Explorer/Fate) accept
   `(hole, context)` parameters. A `collision` IS a hole: it's an
   under-determined morphism awaiting resolution. The five models
   each produce a candidate strategy by their model-specific lens:

   - **Abyss** (focus, depth): reads the AST depth of each source
     and proposes `delete_smaller` if dominance is clear.
   - **Introject** (project, pattern): reads the import-graph and
     proposes `supersede` if the older source has no incoming edges.
   - **Cartographer** (split, map): reads the namespace declarations
     and proposes `disambiguate_via_namespace` if the declared tags
     are distinct.
   - **Explorer** (zoom, explore): proposes `keep_both` with
     candidate path renames (the exploration produces the suffix
     choices).
   - **Fate** (refract, select): proposes `union` if the structural
     diff is `NearIdentical`.

   Each model's `\` body becomes a concrete proposal returning a
   `strategy` value. This is the wiring sub-tick 4b.3.5 closes
   alongside this spec.

### 4.2 What the agent needs Fate to return

For one tournament round, the agent expects:

```
@fate.infer(collision_oid) -> imperfect([strategy], no_proposal, loss)
```

The returned `[strategy]` is a `Vec` of candidate ASTs in the
`@kintsugi/merge.strategy` grammar — one per Fate model that
produced a proposal. Models that decline return `dimmed(no_proposal,
0.0)` (per `kintsugi-formatter.md` stage 1). The agent then:

1. Calls `enumerate(collision)` to compute the applicable set.
2. Intersects Fate's proposals with the applicable set.
3. Discards proposals that the static enumerator says don't apply.
4. Returns the remaining set as the tournament input.

The intersection is conservative: Fate may propose strategies that
look good linguistically but are structurally inapplicable (e.g.
`inline` when namespaces don't match). The static `enumerate`
filter catches these before they enter the tournament.

### 4.3 The Fate prompt shape

Fate's models are not LLMs in the language-model sense; they are
the five Prism implementations defined in `prism/core/src/bundle.rs`
(per `eigenboard-representation.md` §"Fate↔operation mapping"). The
"prompt" is a structural input: the collision's content-OID and
the local sheaf context.

The call:

```
@fate.infer(
  hole_oid:   content_oid(collision),
  context:    eigenboard_section_at(collision_location),
) -> imperfect([strategy_ast], no_proposal, loss)
```

Returns up to 5 strategy ASTs (one per model, modulo declines).
Response parsing: each ast must satisfy
`apply_h(@kintsugi/merge.strategy_grammar, ast)` (well-formedness
check per FP2/FP3). Malformed responses are dropped with a logged
failure (Fate model misbehaved; the static enumerator's strategies
still cover the round).

### 4.4 What's missing — concrete wiring tasks

| Task | Where | Today | After this spec |
|------|-------|-------|------------------|
| `mirror fate` subcommand | `bootstrap/src/main.rs::cmd_fate` | none | accepts `<collision_oid>`; calls `@fate.infer`; emits `[strategy]` |
| `mirror_fate` MCP tool | `bin/mirror-mcp` | advertised, unwired | wired to `cmd_fate` |
| `@fate.infer` grammar | `boot/std/fate.mirror` | missing — only `@fate.tournament` exists (retires per §4.1 reframe) | declared with signature `(hole_oid, context, config) -> imperfect([candidate], no_proposal, loss)`; `config` carries `tournament_bracket(rule)` for the tournament case |
| `@fate.tournament_bracket` config | `boot/std/fate.mirror` | does not exist | absorbs today's `@fate/tournament.rule` sum type as a config value, not a separate action |
| Five models' `\` bodies | `boot/std/ai/*.mirror` | each is `\` | each returns its model-specific strategy proposal |
| `@kintsugi/merge` grammar | `boot/std/kintsugi/merge.mirror` | does not exist | declared per §2.1 |
| Strategy applicability filter | `@kintsugi/merge.enumerate` | does not exist | implements §2.4 matrix |
| Lex composer | `@kintsugi/merge.compose_lex` | does not exist | implements §3.2 |

None of these is a Rust-level change: every declaration is grammar.
The bootstrap absorbs them via the staircase (per
`project-mirror-compile-staircase`). The wiring sub-tick lands
these in order; the tournament's first end-to-end run is the
proof that the staircase reached this step.

---

## 5. The tournament harness

### 5.1 Algorithm (pseudocode)

```
fn tournament_merge(collision: Collision, rule: TournamentRule) -> Plan {
    # Stage 1 — propose. Fate's five models fan out.
    let fate_proposals = @fate.infer(content_oid(collision),
                                     eigenboard_section_at(collision)).candidates
    # Stage 1' — static enumeration.
    let static_proposals = @kintsugi/merge.enumerate(collision)
    # Intersect: Fate's proposals filtered by static applicability.
    let candidates = intersect(fate_proposals, static_proposals)

    if candidates.is_empty() {
        return Plan.fail(no_proposal, collision)
    }

    # Stage 2 — score. Apply each strategy, measure κ.
    let scored = candidates.map(|s| {
        let plan = @kintsugi/merge.apply(collision, s)
        let kappa = @hash/coincidence.holonomy(plan.post_merge_state)
        (s, plan, kappa)
    })

    # Stage 3 — elect.
    # Apply gates C1, C2, C3 from §3.1 — eliminate failures.
    let alive = scored.filter(|(_, plan, _)| {
        plan.passes_kintsugi_reachability()
            && plan.namespace_integrity_loss() == 0
            && plan.xref_integrity_loss()      == 0
    })

    if alive.is_empty() {
        return Plan.fail(all_strategies_fail_gates, collision)
    }

    # Rank by tier 4 (holonomy), then tier 5 (oid churn).
    alive.sort_by(|(_, plan_a, kappa_a), (_, plan_b, kappa_b)| {
        if (kappa_a - kappa_b).abs() < EPSILON_HOLONOMY {
            plan_a.oid_churn().cmp(plan_b.oid_churn())
        } else {
            kappa_a.partial_cmp(kappa_b).unwrap()
        }
    })

    let (winner_strategy, winner_plan, winner_kappa) = alive[0]

    # Stage 4 — verify. The formatter's stage 4 walks obligations.
    let verdict = @epistemologic/property.verify_all(winner_plan)
    if !verdict.all_pass() {
        return Plan.fail(verification, winner_plan, verdict)
    }

    # Stage 5 — fixed-point check (Lawvere).
    if !@epistemologic/math/lawvere.is_fixed_point(
        section_after(winner_plan),
        @kintsugi/tournament.endomap,
    ) {
        # The section is not at a Lawvere fixed point yet.
        # Apply the plan and recurse with the new section as input.
        let new_section = apply_plan(winner_plan)
        return tournament_merge(detect_collisions(new_section), rule)
    }

    return winner_plan
}
```

### 5.2 Bracket structure

**Pairwise elimination, single round.** The strategy set is small
(≤ 6 candidates after enumeration). A bracketed tournament with
multiple rounds is overkill; single-round lex-ordered ranking
suffices. The structure:

```
candidates → gates (parallel) → ranked (sort) → winner
```

**When would we need multi-round?** If the strategy vocabulary
extended to combinators (open vocabulary) with hundreds of
candidates. Today the closed enum keeps the candidate set ≤ 6;
the tournament is a sort, not a bracket.

**The bracket rule `elite(1).beam(8).halving(3)`** IS load-bearing
for *au candidate selection within stage 1*, not for strategy
elimination after stage 2. Two layers compose:

- **Inner layer** (configured, not a separate call): each Fate
  model tournament-selects au candidates inside its own search.
  The bracket rule is the `config: tournament_bracket(...)` value
  passed to `@fate.infer` — a per-call configuration parameter, not
  a distinct API surface. Output: one strategy proposal per model.
- **Outer layer** (this spec's algorithm): the five-or-fewer model
  proposals + static enumerations are gated and ranked
  lexicographically. Output: one winner.

The outer layer is *this spec*. The inner layer is a configuration
field on `@fate.infer`; today's `@fate/tournament.tournament` action
retires in favour of `@fate.infer(…, config: tournament_bracket(…))`
per §4.1. The `elite/beam/halving/tabu/anneal/ucb` sum type that
describes the bracket structure remains, just relocated as a value
type inside `@fate`.

### 5.3 Termination

The tournament terminates by the formatter's Banach contraction
argument (per `kintsugi-formatter.md` §"Convergence guarantee"):

- Each round's winner reduces the holonomy by ≥ `γ` (the bundle's
  spectral gap).
- Holonomy is bounded below by zero.
- Therefore the round count is bounded above by `log(holonomy_0/ε) / log(1/γ)`.

In practice, for a single collision with ≤ 6 candidates, the
tournament terminates in O(1) — one stage-2 measurement per
candidate, one sort, one verification, one fixed-point check. The
formatter's loop wraps this; the outer loop runs until no
collisions remain.

### 5.4 Determinism

The tournament is deterministic up to:

1. The fixed-point check's tie-breaking seed (derived from the
   collision content-OID, so replayable).
2. Fate's model proposals (today: deterministic per model weights;
   future: stochastic per the `beam(k)` rule). The `elite(1)` arm
   guarantees the highest-conductivity proposal is preserved across
   stochastic rounds.

Determinism is load-bearing for the audit trail: Reflection's
recorded morphism must replay byte-identically if re-applied to the
same collision in the same gestalt context.

---

## 6. The kintsugi CLI extension

### 6.1 New flag — `--resolve=<mode>`

```
mirror kintsugi <path> [--shatter N] [--transform <mq>] [--out <dir>] [--resolve=<mode>]
```

`--resolve=<mode>` controls collision behaviour:

- `--resolve=fail` (**default**, today's behaviour) — error on first
  collision, list the colliding sources, exit 1.
- `--resolve=tournament` — invoke `@kintsugi/tournament.resolve` on
  each collision; apply the winning strategy; continue the migrate.
- `--resolve=interactive` (future) — pause on each collision, present
  the ranked strategies with their κ scores, accept user choice.
  Out of scope for this spec; declared as a placeholder.
- `--resolve=fate-only` (debug) — invoke Fate's five models, print
  their proposals (no static enumeration, no gating, no application).
  For agent-level inspection of the proposal layer.

### 6.2 Where it sits relative to existing flags

The flags compose:

```
mirror kintsugi boot/ --transform='grammar => glass' --out=mirror/ --resolve=tournament
```

Reading order: `kintsugi` enters the formatter loop; `--transform`
applies rewrite rules before tokenize; `--out` specifies migrate
mode; `--resolve=tournament` controls collision behaviour during
migrate.

`--shatter N` is orthogonal: it controls recursive depth of the
formatter's inner loop (per `kintsugi-formatter.md` and
`@kintsugi/shatter.fracture_and_repair`). The tournament runs
within each shatter level.

### 6.3 Default — `--resolve=fail`

Deliberately conservative. The tournament is non-trivial machinery;
running it by default would surprise users. The opt-in flag matches
the `--shatter N` and `--liquid` precedent (per
`bin/mirror-mcp`'s declared schema): non-default modes are explicit.

### 6.4 Backward compatibility

No breaking change. Existing `mirror kintsugi <path>` invocations
behave exactly as before (fail-on-collision). The new flag adds
behaviour; it does not change the default.

### 6.5 MCP surface

The `mirror_kintsugi` MCP tool gains a `resolve` parameter:

```json
{
  "name": "mirror_kintsugi",
  "inputSchema": {
    "type": "object",
    "properties": {
      "file": { "type": "string" },
      "liquid": { "type": "boolean" },
      "shatter": { "type": "integer" },
      "resolve": {
        "type": "string",
        "enum": ["fail", "tournament", "interactive", "fate-only"]
      }
    },
    "required": ["file"]
  }
}
```

The `bin/mirror-mcp` script passes `--resolve=<value>` through to
the binary. Default: `fail` (preserves today's behaviour).

---

## 7. Reflection's role post-tournament

A tournament outcome IS a morphism in the eigenboard sheaf. Per
`project-eigenboard-is-sheaf`, morphisms compose by Tambara module
composition; Reflection writes them as `@cogito.strategy` rewrites
on the section.

### 7.1 The gestalt entry

Each tournament round produces a gestalt entry: a typed record
Reflection writes to the eigenboard's section history.

```mirror
type tournament_outcome = {
  collision:        collision,          # the hole
  candidates:       [strategy],         # what was considered
  eliminated:       [(strategy, reason)],
  winner:           strategy,           # what was applied
  winner_kappa:     f64,                # the holonomy of the winning plan
  winner_plan:      plan,               # the file ops
  morphism:         section_morphism,   # the bundle automorphism
  tick:             u64,
  agent:            ref,
  parent_section:   section_oid,        # pre-merge state
  child_section:    section_oid,        # post-merge state
}
```

This is the **typed record of one resolved choice point.** It lives
at `refs/eigenboard/<agent>/tournaments/<oid>` and is referenced from
the parent section's ancestor chain.

### 7.2 Reflection's observation

Reflection's `observe(beam_n, beam_n+1)` call (per
`eigenboard-representation.md` §"`@cogito.observe(beam_n, beam_n+1)`")
gains a typed slot for tournament outcomes:

```mirror
type observation = {
  delta:        section_delta,
  holonomy:     f64,
  loss_delta:   f64,
  tournaments:  [tournament_outcome],   # new in this spec
}
```

The ordered list `tournaments` records every collision resolved
between `beam_n` and `beam_n+1`. Reflection can then:

1. Identify recurring collision patterns (same shape, repeated
   resolution).
2. Surface them to `@cogito.strategy` as candidates for proactive
   resolution.
3. Detect when a strategy that was applied no longer applies (the
   bundle has evolved).
4. Learn weights for Fate's five models (which model proposed the
   winner most often).

None of these is implemented today. The spec declares the shape;
the wiring lands as a Reflection follow-up tick.

### 7.3 The morphism as section automorphism

A tournament outcome's `morphism: section_morphism` field is the
bundle automorphism that takes the pre-merge section to the post-
merge section. Per `eigenboard-representation.md` §"Match modifiers →
bundle automorphisms", a section automorphism is a `match(split)` —
the operation that respects the gauge and chooses among candidates.

The tournament is one *kind* of `match(split)`. Other kinds include
the inner Fate tournament (per `boot/std/fate/tournament.mirror`)
and future au-candidate selection within `match(refract)`.

Reflection composes these automorphisms by Tambara module composition
(per `@epistemologic/math/category`). The eigenboard's section
history IS the composition chain. The chain's content-OID is the
memory of every choice point Reflection has resolved.

### 7.4 The diff/review surface

Future tick: a `mirror review tournament <oid>` command that loads a
tournament outcome and renders the pre/post sections with the
eliminated candidates as alternative diffs. The user can:

- Confirm the winner.
- Pick an eliminated candidate to apply instead (rewinds the
  morphism, applies the alternative).
- Annotate the gestalt with a note that adjusts Fate's model weights
  for the next tournament of similar shape.

This is out of scope for the current spec; it's the user-facing
dressing on the underlying machinery.

---

## 8. Acceptance criteria

The **real tick** that implements this spec passes when:

### 8.1 Collision 1 — `nl.mirror`

```
mirror kintsugi boot/ --transform='grammar => glass' --out=mirror/ --resolve=tournament
```

produces, without hand-intervention:

- `mirror/nl.mirror` containing the contents of `boot/std/mirror/nl.mirror`
  (the rich grammar with bare/markdown/code-inline forms).
- A gestalt entry at `refs/eigenboard/<agent>/tournaments/<oid>`
  recording `winner = disambiguate_via_namespace(boot/std/mirror/nl.mirror)`.
- `mirror/nl/interface.mirror` does NOT exist (the stub at
  `boot/std/nl.mirror` was eliminated by the winning strategy).
- The migrate exit code is 0.

### 8.2 Collision 2 — `runtime.mirror`

The same command produces:

- `mirror/runtime.mirror` containing the contents of
  `boot/std/mirror/runtime.mirror` (the active pipeline grammar).
- A gestalt entry recording `winner = supersede(boot/std/runtime.mirror)`.
- `boot/std/runtime.mirror` is no longer referenced anywhere in the
  migrated tree.
- The migrate exit code is 0.

### 8.3 Synthetic collision — `code/X.mirror`

Designed by the implementing agent. Two files declaring the same
grammar `@code/X` with disjoint actions; the test expects `inline`
to win (same namespace + disjoint actions match the strategy's
applicability condition; the κ is lower than `keep_both`).

The test:

```
# fixture: tests/fixtures/collision-inline/
#   boot/std/code/X.mirror      grammar @code/X { action a -> u64 { \ } }
#   boot/std/mirror/code/X.mirror grammar @code/X { action b -> u64 { \ } }

mirror kintsugi boot/ --transform='grammar => glass' --out=mirror/ --resolve=tournament
```

produces `mirror/code/X.mirror` containing both `a` and `b` actions
under one `@code/X` declaration. The gestalt entry records
`winner = inline(boot/std/mirror/code/X.mirror, boot/std/code/X.mirror)`.

### 8.4 Replay invariance

Running the same migrate twice in a row, on a clean tree, produces
byte-identical output the second time (no work to do; the section is
at a Lawvere fixed point). The tournament has terminated.

### 8.5 Gestalt audit

The gestalt's tournaments ref records:

- The collision content-OID.
- The full strategy enumeration (six candidates).
- The Fate model proposals (which models proposed which strategies).
- The gating verdicts (C1/C2/C3 per candidate).
- The ranked κ values.
- The winner.
- The applied plan.

The audit trail is replayable: re-running the tournament on the
same collision in the same eigenboard section produces a
byte-identical gestalt entry (the deterministic seed is the
collision content-OID).

### 8.6 Failure mode — `all_strategies_fail_gates`

A synthetic third collision designed so every applicable strategy
fails at least one gate. Expected behaviour: the migrate exits 1
with `failure: all_strategies_fail_gates(collision)` and the gestalt
records the failure for human review. The tree is unchanged (the
formatter's honest-failure mode, per `kintsugi-formatter.md` §"Honest
failure is success").

---

## 9. Open questions for Alex

1. **Strategy seventh slot?** The closed enum has six variants today.
   Is there a natural seventh? Candidates considered and rejected:
   `interactive_prompt` (delegates to the user — not a strategy, a
   meta-strategy); `defer` (skip this collision for now — handled by
   the failure mode `all_strategies_fail_gates`); `automorphism`
   (apply a Reflection-proposed bundle automorphism — out of scope,
   that's wire 7 not wire 3). Confirm six is the right closure?

2. **`ε_holonomy` value.** The tier-4/tier-5 boundary is
   `|κ_a - κ_b| < ε_holonomy`. What's the right ε? The bundle's
   spectral gap suggests `ε ≈ λ_2 / λ_1` (the ratio of second-smallest
   to smallest nonzero eigenvalue of the connection Laplacian). For
   the boot tree today, λ_1 ≈ 0.05; λ_2 ≈ 0.18; `ε ≈ 3.6`. That feels
   too large. Alternative: `ε = 0.01` (one percent of the maximum
   possible κ). Which?

3. **`@fate.infer` as the single surface.** [DECIDED 2026-05-22.]
   Alex: "@fate.infer ought to be the single surface for running
   fate. The tournament is a configuration concern in the type
   passed to infer." The earlier framing of
   `@fate.tournament.tournament` as a separate action retires;
   today's `@fate/tournament` grammar collapses into a `config`
   value (`tournament_bracket(rule)`) inside `@fate.infer`. The
   doubled `tournament` was a structural redundancy. See §4.1's
   reframe block for the unified grammar declaration; §4.4's wiring
   table for the absorption tasks; §5.2 for the inner/outer-layer
   reformulation. No follow-up needed on this question.

4. **`mirror fate` subcommand vs `mirror_fate` MCP tool.** Are they
   strictly equivalent surfaces (the MCP tool wraps the subcommand)?
   The road-to-1.0 audit notes both as TODO. If they're equivalent,
   wire one and proxy the other.

5. **Inner-vs-outer tournament composition.** §5.2 names two
   tournaments: Fate's au-candidate selection (inner) and the
   strategy elimination (outer). The inner is `elite(1).beam(8).halving(3)`;
   the outer is lex-ordered. Should the inner tournament's rule
   be tunable per call (e.g. `--resolve=tournament:elite(1).beam(16)`)
   or fixed?

6. **Where do gestalt tournament entries live across worktrees?**
   `refs/eigenboard/<agent>/tournaments/<oid>` is the local path.
   Spectral handles multi-host; do tournament entries replicate
   automatically with the rest of the eigenboard, or are they local-
   only (audit trail of *this* agent's choices)?

7. **Confluence of strategy application.** If two collisions are
   resolved in a different order, do they produce the same final
   section? Intuition: yes for non-overlapping collisions; possibly
   no when one collision's winning strategy depends on the other's
   resolution. Worth confirming with a worked example before
   declaring the tournament confluent.

8. **Failure-mode escalation.** When `all_strategies_fail_gates`
   fires, should the system fall back to `--resolve=interactive`
   automatically (paginate the failure to the user) or hard-fail
   (today's behaviour)? My read: hard-fail today; revisit when
   `--resolve=interactive` lands.

9. **Tournament outcome as content-addressable Prism.** A
   `tournament_outcome` IS a typed Prism on the section (its
   `morphism` field). Should we declare `@kintsugi/tournament.outcome`
   as a grammar that implements `Prism`, making outcomes composable
   via `compose_a`? My read: yes, but in a follow-up tick — this
   spec lands the round, the composition is downstream.

10. **Default behaviour at v1.** Should `--resolve=tournament` become
    the default at v1.0 (and `--resolve=fail` become an opt-in for
    strict CI mode)? Or do we keep `fail` as the default forever?
    My read: tournament becomes default at v1.0 *after* the
    `--resolve=interactive` mode lands (gives users an escape hatch).

11. **Generalisation to non-file collisions.** §1.1 lists six other
    shapes of under-determined morphism (namespace conflicts, type
    redefinitions, etc). Are these in scope for the same tournament
    mechanism, or do they each get their own strategy vocabulary?
    My read: same mechanism, different vocabularies — each shape
    declares its own `@kintsugi/merge/<shape>.mirror` with its own
    closed enum.

12. **Fate's five-model contracts.** §4.1's mapping (Abyss →
    `delete_smaller`, Introject → `supersede`, etc.) is one
    assignment. Should it be load-bearing (each model OWNS a
    strategy family) or advisory (any model can propose any
    strategy)? Load-bearing matches the
    `eigenboard-representation.md` §"Fate↔operation mapping";
    advisory keeps the tournament open to model improvement. My
    read: load-bearing at v0, advisory at v1+ once weights are
    learned.

---

## 10. Cross-spec implications

### 10.1 `kintsugi-formatter.md` (mirror)

Reinforced. The tournament-merge IS the formatter applied at
file-system identity granularity. Stage 1 (propose) maps to §4's
Fate proposal channel + §2.4's static enumeration. Stage 2 (measure)
IS §3's scoring function. Stage 3 (elect) IS §5.1's gates + sort.
Stage 4 (verify) is the property-check sweep on the winner. Stage 5
(fixed-point) is the Lawvere check on the post-merge section.

This spec is the *first concrete consumer* of the formatter's five
stages outside the AST level.

### 10.2 `eigenboard-representation.md` (mirror)

Reinforced. A tournament outcome IS a section morphism — the spec's
new `tournament_outcome` type contains a `morphism: section_morphism`
field. The morphism is a `match(split)` automorphism per
`eigenboard-representation.md` §"Match modifiers → bundle
automorphisms".

The gestalt entry persists across ticks at
`refs/eigenboard/<agent>/tournaments/<oid>`, joining the ancestor
chain Reflection walks. The eigenboard's section history gains a
typed slot for tournament outcomes.

### 10.3 `kintsugi-wiring.md` (mirror)

This spec is the **concrete realisation of wires 2 and 3** for the
file-system-collision shape. Wire 2 (Fate fan-out) is §4.3's
`@fate.infer(collision_oid, context)` call. Wire 3 (conductivity
contest) is §3's scoring function composed lexicographically.

The wires are no longer `\` for the collision case; they have
bodies expressible in the merge-strategy grammar.

### 10.4 `kintsugi-collapse.md` (spectral)

Reinforced. The `merge_grammars` function defined there is the
implementation of the `union` strategy variant. The structural-diff
taxonomy (`Identical` / `NearIdentical` / `Different`) is the
strategy-applicability filter for `union`. The loss-delta computation
is one input to §3's scoring (specifically component C5, holonomy).

### 10.5 `mirror-kintsugi.md` (spectral)

Reinforced. The fixture-based Fate tournament described there (5
Fate models, model-selects-strategy mapping) is the inner tournament
of §5.2. The Fate-model-to-strategy assignment in §4.1 of this spec
matches the model-to-simplification-strategy table in
`mirror-kintsugi.md` §3.

### 10.6 `au-and-conductivity.md` (mirror)

Reinforced. au IS the type Fate produces. Each strategy proposal is
an au candidate; each candidate's κ is its conductivity index. The
tournament's elimination IS the conductivity contest. The winning
strategy's au is the gold that conducts through the post-merge
section.

### 10.7 `parser-as-prism-grammar.md` (mirror)

Reinforced. The post-merge tree must satisfy FP1/FP2/FP3. C1
(kintsugi reachability) is the FP-equivalent test at the migrate
level: `tokenize(render(file)) == file` for every migrated file.
The tournament does not weaken the parser-as-prism guarantee; it
preserves it (gate C1 is mandatory).

### 10.8 `mirror-compile-bootstrap.md` (mirror, Spec A)

This spec rides the io-binding staircase. The `@kintsugi/merge`
grammar's `\` bodies are obligations the staircase will close in
later ticks. The first concrete obligation is the merge-vocabulary
declaration; the wiring sub-tick lands `enumerate` and `apply`;
Fate's models close their `\` per §4.1's mapping.

---

## 11. Out of scope

- **The interactive mode.** `--resolve=interactive` is named but
  not specified. Future tick.
- **The diff/review surface.** `mirror review tournament <oid>` is
  sketched but not specified. Future tick.
- **Cross-host tournament replay.** Spectral handles multi-host;
  this spec is single-process.
- **Strategy combinators (open vocabulary).** §2.3 decides against
  open combinators today; revisit if the closed enum proves
  insufficient.
- **Reflection's learning loop.** §7.2 sketches what Reflection does
  with tournament outcomes; the implementation is its own tick.
- **The au-tissue extension to non-static collisions.** §1.1 lists
  six shapes of under-determined morphism; this spec focuses on the
  file-system case. The other shapes inherit the tournament shape
  but each declares its own `@kintsugi/merge/<shape>.mirror`.
- **The Fate model weights.** §4.1's mapping (Abyss → delete_smaller,
  etc.) is the load-bearing v0 assignment; learning the weights
  from gestalt history is a future Reflection tick.
- **Pareto-front composition as alternative.** §3.2 commits to lex.
  If lex proves brittle (e.g. tier-4 ties are common), revisit with
  a pareto-front + manual-selection fallback. Today: lex is the
  cheaper choice.
- **Performance.** A tournament round is O(|candidates| × |criteria|);
  candidates ≤ 6, criteria = 5. Constants are small. No optimisation
  in this tick; revisit if a real workload (multi-thousand file
  migrate) shows the constants matter.
- **The metric on `section_morphism`.** Reflection composes morphisms
  by Tambara module composition; a metric on the composition group
  would let us measure "how far" Reflection has steered. Future
  tick.

---

## 12. References

### Mirror corpus

- `mirror/docs/specs/kintsugi-formatter.md` — the contraction-map
  framing.
- `mirror/docs/specs/eigenboard-representation.md` — the bundle
  substrate.
- `mirror/docs/specs/kintsugi-wiring.md` — the eight wires; this
  spec realises wires 2–3 for the collision case.
- `mirror/docs/specs/au-and-conductivity.md` — au IS Fate's output;
  conductivity IS the scoring axis.
- `mirror/docs/specs/parser-as-prism-grammar.md` — FP1/FP2/FP3;
  gate C1 (kintsugi reachability) preserves them.
- `mirror/docs/specs/mirror-compile-bootstrap.md` (Spec A) — the
  io-binding staircase this spec rides.
- `mirror/docs/specs/match-select.md` — `match(split)` modifier; the
  tournament's outcome is a `match(split)` automorphism.
- `mirror/docs/specs/lsp-and-mcp.md` — the MCP tool surface;
  `mirror_kintsugi` gains a `resolve` parameter.
- `mirror/docs/specs/road-to-1.0.md` — Tick 4 (MCP cleanup);
  `mirror_fate` advertised-but-unwired today.
- `mirror/docs/ai/tournament.md` — the rule vocabulary (greedy /
  beam / elite / halving / tabu / anneal / ucb).
- `mirror/boot/std/fate/tournament.mirror` — the inner-tournament
  grammar.
- `mirror/boot/std/ai/*.mirror` — the five model grammars.
- `mirror/boot/std/kintsugi.mirror` — the public kintsugi entry.
- `mirror/boot/std/kintsugi/migrate.mirror` — the migrate grammar
  (today: body `\`; this spec gives it a concrete shape).
- `mirror/bootstrap/src/main.rs::cmd_kintsugi_migrate` — the migrate
  loop; the collision is detected here today.

### Spectral corpus

- `spectral/docs/specs/kintsugi-collapse.md` — `merge_grammars` is
  the `union` strategy's implementation.
- `spectral/docs/specs/mirror-kintsugi.md` — the Fate-tournament test
  fixture; original model-to-strategy mapping.
- `spectral/docs/specs/spectral-spawn.md` — the actor model; agents
  resolve collisions in their own gestalt context.
- `spectral/docs/specs/agent-eigenboard-spec.md` — the eigenboard's
  agent-specific section.
- `spectral/docs/specs/gutter-lenses.md` — the five dualities the
  scoring function maps to.

### Reed memory

- `project-eigenboard-is-sheaf` — cellular sheaf; morphisms;
  restriction maps; Reflection writes the morphisms.
- `project-au-conductivity` — Fate's output type; conductivity
  contest; the tournament IS the contest.
- `project-mirror-compile-staircase` — the order this spec lands
  in.
- `feedback-substrate-pull` — logic in grammar, not Rust; merge
  vocabulary is grammar.
- `feedback-no-stringly-types` — `strategy` is a typed enum, not a
  string.
- `architecture` — the five operations; tournament is a
  `match(split)` instance.

---

*A collision is an under-determined morphism.*
*The tournament is the formatter applied to identity.*
*Six strategies; one closed sum type in mirror; the scorer is sub-Turing.*
*Lexicographic composition over gates and ranks.*
*One Fate surface — `@fate.infer` — with the tournament as a config.*
*Fate's five models propose; the static enumerator filters;*
*the gates eliminate; the holonomy ranks; the OID tiebreaks.*
*The winner is a bundle automorphism; the gestalt records it.*
*Reflection observes; the next tournament learns;*
*the morphism composes; the eigenboard converges.*
*Honest failure is success; the residue is returned.*
*When the section reaches the Lawvere fixed point, the migrate is done.*

Apache-2.0.
