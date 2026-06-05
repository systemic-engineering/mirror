# CLI as Prism — Condensation Under the Recursive Five-Operation Constraint

*2026-06-05. Reed + Alex. Spec (exploration, not implementation).*

Status: **Red** (proposal).
Branch: `reed/shards-floor`.
Continues: `cybernetic-cli.md` (the surface this condenses), `mosaic.md` (the
`glass @mirror/<x>` precedent, C2).
Depends on: `shards/mirror/cli.mirror` (C1 — the substrate-level prism at
`@mirror/cli`), the path-namespace property (epistemologic B5), [[architecture-prism-as-trait-as-everything]],
[[architecture-lift-as-load-bearing]].

---

## 0. The constraint, stated once

> The mirror CLI is a **prism**, with the five operations as its direct
> interface. Each subcommand is a **glass** (hence again a prism), with its
> own five operations. Sub-glasses nest recursively. The file structure on
> disk IS the CLI structure: `shards/mirror/cli/<x>.mirror` declares the
> glass that handles `mirror <x>`.

This is exactly the shape that landed at the **build altitude** in C2
(`shards/mirror/mosaic.mirror`): the build system was condensed into the
five operations on the project manifold. We are now applying that same
shape **recursively to the CLI itself**, one altitude up.

The constraint is **soft**. It earns its keep where it earns its keep.
Where it strains, we name the strain. The strain is the finding.

---

## 1. The recursion, named

### 1.1 Top level — `@mirror/cli` is a prism

`shards/mirror/cli.mirror` already declares (C1):

```mirror
prism @mirror/cli {
  focus    cli
  project  cli
  split    cli
  shift    cli
  settle   cli
}
```

This is the **direct interface**: typing `mirror focus`, `mirror project`,
`mirror split`, `mirror shift`, `mirror settle` runs the top-level operation
on the **project manifold** (the resolved `mirror.spec`). Same algebra as
everything else; the manifold here is the CLI invocation against the project.

```
mirror focus    [target...]   — observe the project manifold as-is
mirror project  [predicate]   — filter the manifold by a predicate
mirror split    [edge]        — follow connectivity in the manifold
mirror shift    [functor]     — same bytes, declared at a new altitude
mirror settle   [optic]       — the ONE write; produces a proof block
```

This is also the plumbing surface from `cybernetic-cli.md` §2.1 — unchanged.
Plumbing was already in the algebra. The condensation question is what
happens to the **porcelain**.

### 1.2 Sub-glass level — every named verb is a `glass @mirror/cli/<x>`

A subcommand is a **glass within `@mirror/cli`** — hence itself a prism with
its own five operations on its own sub-manifold. The dispatcher walks the
path; the leaf's operation runs.

```
mirror <x>            → focus on glass @mirror/cli/<x>
mirror <x> <op>       → run operation <op> of glass @mirror/cli/<x>
mirror <x> <op> ...   → arguments to <op>
```

Where `<op>` is one of `focus | project | split | shift | settle`. **Always
the same five.** Every glass at every depth.

The "bare" form `mirror <x>` (no operation given) defaults to `focus` of that
glass. Reading is always free; the user pays only for `settle`.

### 1.3 Recursion bottoms out at the leaf operation

`mirror <x> <y> <op>` reads `shards/mirror/cli/<x>/<y>.mirror` (the
nested glass) and runs its `<op>`. The recursion is **as deep as the
substrate demands** — most verbs land at depth 1, a few at depth 2, the
algebra never deeper than 3.

---

## 2. Mapping the cybernetic-cli surface onto the recursion

The cybernetic-cli surface had:

- **Plumbing (5):** `focus`, `project`, `split`, `shift`, `settle`.
- **Porcelain (7):** `compile`, `kintsugi`, `shatter`, `bootstrap`, `converse`,
  `watch`, `reflect`.
- **Third state (4):** `open`, `holes`, `force`, `seal`.
- **Time (1, sketched in the task brief):** `revert`.

**Total: 17 named CLI verbs.**

Each is placed into the recursion below. The placement type is one of:

- **(top-op)** — the verb collapsed into one of the five top-level ops, perhaps
  with a flag.
- **(sub-glass)** — the verb became its own `glass @mirror/cli/<x>` with its
  own five operations.
- **(sub-sub-glass)** — nested two deep.
- **(flag)** — the verb collapsed to a flag-argument of one of the five.
- **(did-not-condense)** — named friction; the verb resists the shape.

### 2.1 Per-verb decisions

| Cybernetic verb | Placement | Form | Notes |
|---|---|---|---|
| `compile` | **sub-glass** | `mirror compile { focus, project, split, shift, settle }` | A NAMED LOOP, not a single op. Top-level `settle` is the single write; `compile` is the canonical pipeline (`focus → project → shift → settle`) given a name because users say "build" more often than they say "settle store after shift @code/rust". Sub-glass `compile.settle` IS the build; `compile.focus` previews what would change. |
| `kintsugi` | **sub-glass** | `mirror kintsugi { focus, project, split, shift, settle }` | The coherence-settling loop. `focus` = peek at the next tournament move. `project` = filter by what to repair (target, eigenvalue range). `split` = explore candidate fills. `shift` = basis-transform on the loss landscape. `settle` = run one tournament iteration AND write. **Sings.** The five ops are the loop's verbs. |
| `shatter` | **sub-glass** | `mirror shatter { focus, project, split, shift, settle }` | Materialize AST → text. `focus` = show what would shatter. `project` = filter to one target altitude. `split` = enumerate materialization variants. `shift` = re-emit at a different altitude. `settle` = write the `.shatter` file (the projection of `au`). Could have been `mirror shift --emit shatter` at the top level, but shatter has its own lifecycle (the `.shatter` file is a first-class artifact per cybernetic-cli §3). Sub-glass earns it. |
| `bootstrap` | **sub-glass** | `mirror bootstrap { focus, project, split, shift, settle }` | The autopoietic phase machine (7 phases). `focus` = report phase status. `project` = filter to one phase. `split` = enumerate sub-phase dependencies. `shift` = shift between phase representations (eigenvalue / line count / proof). `settle` = advance one phase (the ONE write; proof block records the transition). **Strains on `split` slightly** — phases don't naturally branch, they sequence. Named in §4. |
| `converse` | **sub-glass** | `mirror converse [@peer] { focus, project, split, shift, settle }` | Second-order CLI. Peer-id is a **path component**: `mirror converse @reed` reads `shards/mirror/cli/converse.mirror` with `@reed` as the glass's argument. Inside: `focus` = read peer's current pos. `project` = filter the conversation by topic. `split` = branch into a sub-conversation. `shift` = switch perspective (peer ↔ self). `settle` = land an utterance (commits to the transcript). The `λsh` drop-in is `mirror converse` with no peer — the empty path puts the user IN the glass interactively. Earns it. |
| `watch` | **flag** | `mirror focus --watch` | **Did not condense as a sub-glass.** Watch is a TEMPORAL MODE on observation, not its own algebra. There is no "settle the watch" — the watch settles whatever it is watching. Named in §4 as friction-that-resolved-cleanly: the friction taught us watch isn't a verb, it's an adverb. Same shape as `git log --follow`. |
| `reflect` | **sub-glass** | `mirror reflect { focus, project, split, shift, settle }` | VSM System 5 view. `focus` = print eigenboard + spectrum. `project` = filter to one position. `split` = explore correction trajectories. `shift` = view the same state at a different altitude (per-tick, per-project, per-corpus). `settle` = land a correction to `eigenboard.mirror` (the ONE write; algedonic-triggered only per cybernetic-cli §9.5). **Mild redundancy with top-level `focus`** — both observe meta-state. Named in §4. |
| `holes` | **op of `hole` glass** | `mirror hole focus` | `focus` of the hole glass lists every open `\`. The cybernetic-cli's `holes` verb IS the focus of the third-state sub-manifold. |
| `open` | **op of `hole` glass** | `mirror hole settle --open <name>` | `open` is the event of declaring a new `\`. As an event it lands on `settle` (the ONE write). The `--open` flag distinguishes "create a new hole" from "seal an existing one". **Slight strain** — `open` and `seal` are both `settle` differentiated by flag; the typed lambda's arg shape carries the distinction. See §4. |
| `force` | **op of `hole` glass** | `mirror hole settle --force <name>` | `\!` is force-fill; honest about regression. Same `settle` op, different flag. The flag distinguishes "accept the loss, force the fill" from "wait for natural convergence". |
| `seal` | **op of `hole` glass** | `mirror hole settle <name>` | The bare `settle` IS seal — promote a converged `\` to explicit. The cleanest collapse in the bunch. |
| `revert` | **sub-glass** | `mirror revert { focus, project, split, shift, settle }` | Time-travel. `focus` = show state at tick N. `project` = filter by what changed. `split` = explore alternative timelines branching from tick N. `shift` = re-anchor HEAD to a different tick basis. `settle` = actually revert (writes the new HEAD; proof block records the cost). Works cleanly; revert is genuinely a five-op surface over the time manifold. |

### 2.2 The condensed top-level surface

```
# Five top-level operations on the PROJECT MANIFOLD
mirror focus    [target...]
mirror project  [predicate]
mirror split    [edge]
mirror shift    [functor]
mirror settle   [optic]

# Six sub-glasses, each with the same five operations
mirror compile    [focus|project|split|shift|settle] [args]
mirror kintsugi   [focus|project|split|shift|settle] [args]
mirror shatter    [focus|project|split|shift|settle] [args]
mirror bootstrap  [focus|project|split|shift|settle] [args]
mirror converse   [@peer] [focus|project|split|shift|settle] [args]
mirror reflect    [focus|project|split|shift|settle] [args]
mirror revert     [focus|project|split|shift|settle] [args]
mirror hole       [focus|project|split|shift|settle] [args] [--open|--force]
```

That is **the entire surface**. Five top-level ops. Seven sub-glasses (six
plus `hole`). Every sub-glass has the same five ops. **Same algebra
everywhere.** A user who learns the five operations once knows every command
at every depth.

---

## 3. The on-disk file structure

The path-namespace property (B5) makes the directory layout literal:

```
shards/mirror/
├── cli.mirror                  # prism @mirror/cli — the top-level five ops
└── cli/
    ├── compile.mirror          # glass @mirror/cli/compile { f,p,s,sh,se }
    ├── kintsugi.mirror         # glass @mirror/cli/kintsugi  { f,p,s,sh,se }
    ├── shatter.mirror          # glass @mirror/cli/shatter   { f,p,s,sh,se }
    ├── bootstrap.mirror        # glass @mirror/cli/bootstrap { f,p,s,sh,se }
    ├── converse.mirror         # glass @mirror/cli/converse  { f,p,s,sh,se }
    ├── reflect.mirror          # glass @mirror/cli/reflect   { f,p,s,sh,se }
    ├── revert.mirror           # glass @mirror/cli/revert    { f,p,s,sh,se }
    └── hole.mirror             # glass @mirror/cli/hole      { f,p,s,sh,se }
                                #   (open/force/seal collapse to settle + flag)
```

**Eight `.mirror` files** declare the entire CLI surface. `ls shards/mirror/cli/`
IS the help text — every file is a glass, every glass has the same five
operations. The CLI is **self-describing by directory listing**.

### 3.1 What goes inside a sub-glass shard (sketch)

`shards/mirror/cli/kintsugi.mirror` would declare:

```mirror
in @prism
in @glass
in @mirror/cli
in @nl

# @mirror/cli/kintsugi — the coherence-settling loop as a glass.
#
# Same algebra as @mirror/cli, applied to the kintsugi sub-manifold
# (the tournament's working set). Each op is the same op the parent
# named, restricted to this glass's domain.
#
# Path-namespace property: this file at shards/mirror/cli/kintsugi.mirror
# declares @mirror/cli/kintsugi and only that.

glass @mirror/cli/kintsugi {
  focus    target          # peek the next tournament move
  project  predicate       # filter by what to repair
  split    candidate       # enumerate candidate fills
  shift    basis           # basis-transform on the loss landscape
  settle   iteration       # run one tournament iteration; write proof
}

# Help text via @nl literals lifted from `#` lines (per cli.mirror C1)
focus(target: ref) -> peek    { # show next tournament candidate \ }
project(target, predicate)     { # filter the working set \ }
split(target) -> [candidate]   { # enumerate candidate fills \ }
shift(target, basis) -> view   { # re-view at a different basis \ }
settle(target) -> au           { # run one iteration; emit proof \ }

out @mirror/cli/kintsugi
out focus
out project
out split
out shift
out settle
```

The shape repeats for each sub-glass shard. The boilerplate IS the
substrate — saying "this glass implements the five operations" five times
in a row is the **structure-equals-structure** point. The repetition is
the proof that the recursion held.

### 3.2 Depth-2 (when it's earned)

The recursion can go deeper, but **only when the algebra demands it**.
Sketch of where depth-2 might land:

```
shards/mirror/cli/converse/
  reed.mirror              # glass @mirror/cli/converse/reed
  alex.mirror              # glass @mirror/cli/converse/alex
```

…would make each peer a sub-glass. **This is NOT proposed for v0.1.**
Peers are dynamic; baking them into the on-disk structure is wrong. Instead,
`mirror converse @reed` passes `@reed` as an argument to the
`@mirror/cli/converse` glass's `focus` op. Depth-2 directories are
**reserved** but not minted.

The CLI directory tree is **as deep as the substrate has reason to be**.
Static structure for substrate facts; dynamic args for runtime values.

---

## 4. Where the constraint strained — the findings

The exploration IS the contribution. Six places the constraint pushed back:

### 4.1 `watch` is an adverb, not a verb

`mirror watch` was the cleanest concept in the cybernetic CLI ("Beer's
algedonic bypass made first-class"). It refused to become a glass. There
is no algebra on watching — you don't `project a watch`, you don't `settle
a watch`. The watch settles whatever it observes.

**Resolution: `--watch` flag on `focus`.** `mirror focus --watch` opens
the foreground algedonic surface; the prompt-colour escalation lives in
the renderer, not in a glass. This is the **same shape as `git log
--follow`** — temporal modifiers on read operations are flags everywhere
in CLI history.

**The friction taught us watch isn't a verb.** The cybernetic CLI was
mildly wrong; the recursion surfaced the wrongness. This is the constraint
doing its job.

### 4.2 `open` and `force` are events on `settle`, not separate ops

The third-state lifecycle `holes → open → force → seal` collapsed
beautifully into the `hole` glass with the same five operations:

- `hole focus`   = `holes` (list)
- `hole settle`  = `seal`  (the natural settlement)
- `hole settle --open`  = `open`  (declare a new `\`)
- `hole settle --force` = `force` (`\!`)

But this **flattens three distinct events onto one verb-with-flags**.
The cybernetic CLI's `open / force / seal` names were better at making
the lifecycle visible at the shell. The condensation is structurally
correct (one write op per glass, distinguished by typed-lambda arg shape)
but **costs the user a bit of shell legibility**. `mirror seal X` was
direct; `mirror hole settle X` is one more word.

**Honest tradeoff.** The constraint says "one write per glass." The
ergonomics say "different verbs for different events." We pick the
constraint, but we note the cost: `proof block` and `--open / --force`
flag values have to carry the lifecycle distinction that the verb name
USED to carry. This puts more weight on the proof block (which is fine —
the proof block is where the cybernetic information lives anyway).

### 4.3 `bootstrap` strains on `split`

The 7 phases of self-hosting **sequence**; they don't **branch**. So
`mirror bootstrap split` has no obvious meaning. Candidates:

- "Enumerate the dependencies of the current phase" — works, weakly.
- "Show the phase tree from this point" — works, fits if we think of
  phases as a DAG rather than a sequence.
- "Branch a parallel bootstrap timeline" — only if multi-bootstrap is
  ever a thing (it isn't, currently).

**Resolution: `bootstrap split` enumerates sub-phase dependencies.**
Weak but defensible. The constraint held; the price is that this one op
has a slightly artificial meaning.

**Finding: when the underlying manifold isn't graph-shaped, `split` strains
because `split` IS the graph-walk verb.** Bootstrap is a phase machine
(linear with dependencies). The constraint is honest about graph algebra
being its native habitat. Same-shape-different-substrate works, but the
fit gets looser. Worth naming.

### 4.4 `reflect` overlaps with top-level `focus`

`mirror focus` observes the project manifold. `mirror reflect focus`
observes the meta-state (eigenboard, positions, spectrum). The user
can reasonably ask: "isn't the eigenboard part of the project manifold?
Why are there two ways to observe it?"

**Honest answer: it's a question of altitude.** Top-level `focus`
observes the **substrate state** (what is). `reflect focus` observes the
**identity that observes the substrate state** (who is observing). VSM
System 5 vs System 4. The distinction is structurally real but easy to
miss at first.

**Resolution: keep `reflect` as a sub-glass.** The redundancy is **only
apparent at depth-0**; at depth-1 the sub-manifolds are distinct. But
this is the case the documentation will have to carry most carefully —
the recursion can produce *apparent* duplicates that aren't.

### 4.5 `compile` vs top-level `settle`

`mirror settle store` writes the substrate. `mirror compile <target>` ALSO
writes the substrate. Are they the same?

**No.** Top-level `settle` is the single algebraic operation — one
write, one proof, no loop. `compile` is the **named pipeline** that
expands to `focus → project → shift → settle` and is the canonical
build invocation. The cybernetic-cli explicitly distinguished plumbing
(the algebra) from porcelain (the named loops); the recursion preserves
this: the sub-glass `compile` IS the named loop, while top-level `settle`
is the algebra.

**The strain:** users will sometimes type `mirror settle` when they mean
`mirror compile settle`. The error mode is clean (top-level `settle`
needs an `optic` arg; the bare invocation prints help); but the
near-namesake is a UX cost. **The constraint exposes this cost honestly
rather than hiding it under different names.**

### 4.6 Depth-2 is reserved but not minted

The recursion CAN go arbitrarily deep. The question is when it should.

**Rule (proposed):** depth-N is minted iff the sub-sub-manifold has its
own algebra distinct from its parent's algebra restricted to it. If the
sub-sub-glass's `focus` is just "the parent's focus with one arg pinned,"
it doesn't deserve its own shard — it's an argument value, not a glass.

This is **substrate-pull at the directory layer**. Static structure for
substrate facts. Dynamic args for runtime values. Peers in
`converse`: arguments. Phases in `bootstrap`: arguments (or a flag-enum,
NOT sub-shards `phase-1.mirror`, `phase-2.mirror`).

**Finding: the directory layout is a TYPE. Use it to encode invariants,
not behaviours.** The constraint forces this distinction earlier than the
implementer might otherwise notice. Useful pressure.

---

## 5. Where the constraint sang

For balance, the verbs that condensed elegantly are worth naming too:

### 5.1 `holes/open/force/seal` → `hole` glass

**4 verbs → 1 sub-glass × 1 small flag-set.** The lifecycle stays visible
(via `--open / --force`), the algebra is preserved exactly, the
substrate's third-state vocabulary stays first-class. The cleanest
condensation of all. **The constraint earned its keep here once, no
matter what else happens.**

### 5.2 `kintsugi` as a five-op glass

`focus / project / split / shift / settle` map onto the tournament loop
**without forcing**. `kintsugi.settle` IS one iteration. `kintsugi.focus`
IS peeking the next move. `kintsugi.split` IS enumerating candidate
fills. The tournament was already the same algebra at a smaller scale;
the recursion just **made the algebra visible at the shell**. The
substrate told us this was the right shape.

### 5.3 `revert` as a five-op glass over the time manifold

Time has graph structure (ticks → ticks); the five ops fit cleanly.
`revert.focus tick=N` shows past state, `revert.split` shows branches
diverging from tick N, `revert.settle` does the actual time-travel.
The eigenboard's proof block on `settle` records the cost of the
revert. **No friction.**

### 5.4 `converse` with peer-as-path-argument

`mirror converse @reed` reading the peer as the glass's argument feels
**right** in the way `cd @reed` would feel right in a graph filesystem.
The constraint guided us toward "peers are values flowing through the
algebra," not "peers are sub-substrates." That's a meaningful piece of
architecture clarified by the recursion.

### 5.5 The dispatcher is trivial

The dispatcher is **literally** `walk shards/mirror/cli/<path>.mirror;
load glass; call op`. There is no dispatch table. There is no
sub-router. The file structure IS the dispatch tree. **Substrate-pull at
the entire CLI layer.** This is what was being asked of mosaic at the
build altitude, and the same answer works one altitude up.

### 5.6 Same-five-ops-everywhere is a learnability win

A user who learns `focus / project / split / shift / settle` once knows
every command at every depth. The mnemonic load drops from
**17 verbs** to **5 ops + 7 sub-glass names = 12 names**, and the 5 ops
are already the mental model the substrate is built on. **One vocabulary,
infinite recursion.**

---

## 6. Comparison: cybernetic-cli vs cli-as-prism

|                          | cybernetic-cli (before)       | cli-as-prism (after)         |
|--------------------------|-------------------------------|------------------------------|
| Distinct verb names      | 17 (5+7+4+1)                  | 12 (5 ops + 7 sub-glasses)   |
| Dispatchable leaves      | 17                            | ~37 (5 + 7×5 + flags)        |
| Names-to-learn           | 17                            | 5 ops + 7 sub-glasses        |
| Algebra at every depth   | only at plumbing level        | **at every depth**           |
| File structure ↔ CLI     | implicit (Rust dispatch)      | **literal (path = invocation)** |
| Dispatcher complexity    | match-on-verb table           | walk-the-path                |
| `seal` (cheap to type)   | `mirror seal X`               | `mirror hole settle X`       |
| `holes` (read-heavy)     | `mirror holes`                | `mirror hole focus`          |
| `watch` (modal)          | `mirror watch`                | `mirror focus --watch`       |
| `compile` (canonical)    | `mirror compile T`            | `mirror compile settle T` ⁕  |
| Substrate carries it     | partial (porcelain in Rust)   | **whole CLI in shards/**     |

⁕ With `mirror compile T` aliasing to `compile settle T` via the glass's
default-op rule (bare invocation = focus, BUT `compile` is the named loop
so it can declare `settle` as its default for the action-form). See §7.

**Net:** 5 fewer names to learn. ~20 more dispatchable leaves (because every
glass exposes 5 ops). Substrate carries the entire CLI in 8 `.mirror` files.
The CLI becomes inspectable by `ls shards/mirror/cli/`.

### 6.1 Is the substrate more or less load-bearing?

**More.** The cybernetic CLI had the algebra at the plumbing layer and
named loops in Rust. The recursive form puts the **named loops in the
substrate too** — each sub-glass is a `.mirror` shard with declared types,
declared operations, and obligation blocks. The Rust runtime walks the
substrate; it does not name commands. **Substrate-pull at the CLI layer
is complete** in the recursive form, partial in the cybernetic-cli form.

This is consistent with the broader substrate-pull arc on `reed/shards-floor`:
**logic belongs in grammar, not Rust.** The recursive CLI extends that
arc to the dispatch table.

---

## 7. Default-op rule (the one place the recursion needs a convention)

When a user types `mirror <x>` without specifying an op, what runs?

**Default: `focus`.** Reads are free; reads default. `mirror kintsugi` is
short for `mirror kintsugi focus`.

**Exception: action-named glasses default to `settle`.** A glass whose
NAME is a verb-of-action (`compile`, `revert`, `shatter`) defaults to
`settle` so the bare form does the canonical thing. `mirror compile T`
runs the build. `mirror revert tick=N` actually reverts.

**Rule declared in the glass shard itself** via an explicit `default`
field (per `shards/mirror/cli.mirror` C1's `default(name, t, value)`
pattern):

```mirror
glass @mirror/cli/compile {
  default settle              # action verb — bare form writes
  focus    target
  project  predicate
  split    candidate
  shift    altitude
  settle   target
}

glass @mirror/cli/kintsugi {
  default focus               # named-loop noun — bare form reads
  ...
}
```

This is the **only convention** the recursion needs beyond the constraint.
Everything else falls out of the file structure.

---

## 8. What this spec does NOT propose

- **No new shards in this round.** This is exploration; ticks land later
  on `reed/shards-floor` or its successor.
- **No depth-2 directories.** `shards/mirror/cli/converse/` is reserved
  but not minted. Peers are arguments, not substrate.
- **No alteration to top-level `@mirror/cli`'s five ops.** C1 stands as-is.
- **No dropping cybernetic vocabulary.** `algedonic`, `viable-system`,
  `requisite-variety`, `pattern-that-connects` stay load-bearing in the
  per-glass help text (`#` @nl literals in each shard's body). The
  recursion **carries the cybernetic frame**; it doesn't replace it.
- **No removal of `proof` blocks.** Each glass's `settle` produces a
  proof. This is unchanged; the recursion just multiplies the points at
  which proofs are emitted (each glass's settle produces its own).

---

## 9. Open questions (for Reed + Alex to call before ticks land)

1. **`compile` vs top-level `settle` overlap (§4.5).** Is the action-glass
   default-rule (`mirror compile T` ≡ `mirror compile settle T`) good
   enough, or does the near-namesake demand we rename one? My read: keep
   both; the help text disambiguates; users learn the distinction once.

2. **`bootstrap.split` semantics (§4.3).** Settle on "enumerate sub-phase
   dependencies" or pick a different reading? My read: enumerate
   sub-phase dependencies, accept the weak fit, document the friction.

3. **`hole settle --open / --force` ergonomics (§4.2).** Is the
   verb-with-flag form acceptable, or do we relent and add `mirror hole
   open / hole force / hole seal` as direct ops? My read: keep the flag
   form; the lifecycle moves to the proof block where it belongs anyway.

4. **`reflect` vs top-level `focus` (§4.4).** Keep `reflect` or fold into
   `focus --meta`? My read: keep. The altitude distinction is structurally
   real and `focus --meta` undersells what reflect does (the write path
   to eigenboard.mirror is real and warranted).

5. **`watch` flag scope.** Does `--watch` work only on top-level `focus`,
   or on every glass's `focus`? My read: every glass. `mirror kintsugi
   focus --watch` should work the same as `mirror focus --watch`. The
   adverb composes with the verb at any altitude.

6. **`converse @peer` argument shape.** Does `@peer` come BEFORE or AFTER
   the op? `mirror converse @reed focus` or `mirror converse focus @reed`?
   My read: BEFORE (path-shape). `mirror converse @reed` walks toward the
   peer; the op is what you do once you're there.

7. **Default op declaration syntax.** §7's `default settle` field is a
   sketch. Is the actual grammar `default = settle` or `default settle`
   or `default(op)`? Defer to the substrate's existing default syntax
   (per cli.mirror C1's `default(name, t, value)` typed-lambda form).

---

## 10. The proof

The recursion holds at the CLI altitude **because the same algebra
holds at every altitude** — that's the metapattern. The cybernetic CLI
named this and located the algebra at the plumbing layer. The recursive
form locates it **at every layer the CLI exposes**.

Where the recursion strained (watch as adverb, open/force as flags,
bootstrap on split), the strain **disclosed something true** that the
cybernetic CLI was hiding: watch isn't a verb, open and force are events
on the same write, phase-machines aren't graph-shaped. The constraint
**earned its keep by what it revealed**, not just by what it condensed.

Where the recursion sang (`hole`, `kintsugi`, `revert`, `converse`,
dispatcher-as-path-walk), it **closed a loop** that mosaic opened at the
build altitude: the file structure IS the type, the type IS the
substrate, the substrate IS the algebra, the algebra IS the same five
operations at every altitude. **eⁿ⁺¹ ≤ eⁿ** at the CLI surface too — the
verb count goes down without the expressive surface going down.

The CLI is a prism. The CLI's subcommands are glasses within that prism.
The CLI's sub-subcommands, where the substrate demands them, are glasses
within those glasses. Same five operations. Recursively. All the way
down.

---

*Plumbing was always the algebra.*
*Porcelain WAS named loops — now it's named loops with the same algebra.*
*The file structure on disk IS the CLI structure.*
*The constraint was soft. The breakages were the findings.*
*`focus / project / split / shift / settle` — once, everywhere, recursively.*
