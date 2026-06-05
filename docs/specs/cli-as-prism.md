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
- **Porcelain (7):** `compile`, `kintsugi`, `shatter`, `bootstrap`, `join`,
  `watch`, `reflect`.
- **Third state (4):** `open`, `cracks`, `force`, `seal`.
- **Time (1, dissolved into the `time` glass below):** `revert`.

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
| `join` | **sub-glass** | `mirror join [@peer] { focus, project, split, shift, settle }` | Second-order CLI. **Rename: `converse` → `join`.** Join is what you DO to a conversation; conversing is what happens INSIDE it. Glanville's verb — Conversation Theory names the entry, not the activity. Peer-id is a **path component**: `mirror join @reed` reads `shards/mirror/cli/join.mirror` with `@reed` as the glass's argument. Inside: `focus` = observe peer's eigenboard without entering. `project` = filter the conversation by topic. `split` = branch into a sub-conversation. `shift` = see from their altitude (peer ↔ self). `settle` = enter (land an utterance; commits to the transcript). The `λsh` drop-in is `mirror join` with no peer — the empty path puts the user IN the glass interactively. **Sings** — names the entry, lets the activity inside stay nameless. |
| `watch` | **flag** | `mirror focus --watch` | **Did not condense as a sub-glass.** Watch is a TEMPORAL MODE on observation, not its own algebra. There is no "settle the watch" — the watch settles whatever it is watching. Named in §4 as friction-that-resolved-cleanly: the friction taught us watch isn't a verb, it's an adverb. Same shape as `git log --follow`. |
| `reflect` | **sub-glass** | `mirror reflect { focus, project, split, shift, settle }` | VSM System 5 view. `focus` = print eigenboard + spectrum. `project` = filter to one position. `split` = explore correction trajectories. `shift` = view the same state at a different altitude (per-tick, per-project, per-corpus). `settle` = land a correction to `eigenboard.mirror` (the ONE write; algedonic-triggered only per cybernetic-cli §9.5). **Mild redundancy with top-level `focus`** — both observe meta-state. Named in §4. |
| `cracks` | **op of `crack` glass** | `mirror crack focus` | **Rename: `hole` → `crack`.** `\` produces fracture per gap-tension-tensor-substrate.md (the gap fold). `crack` carries the kintsugi geometry — settle on a crack IS the gold pour. `focus` of the crack glass lists every open `\`. The cybernetic-cli's `holes` verb IS the focus of the third-state sub-manifold. |
| `open` | **op of `crack` glass** | `mirror crack settle --open <name>` | `open` is the event of declaring a new `\` (a new crack). As an event it lands on `settle` (the ONE write). The `--open` flag distinguishes "declare a new crack" from "seal an existing one with gold". The typed lambda's arg shape carries the distinction. See §4. |
| `force` | **op of `crack` glass** | `mirror crack settle --force <name>` | `\!` is accept-loss-and-fill; honest about regression. Same `settle` op, different flag. The flag distinguishes "accept the loss, force the fill" from "wait for natural convergence". |
| `seal` | **op of `crack` glass** | `mirror crack settle <name>` | The bare `settle` IS seal-with-gold — promote a converged `\` to explicit. The kintsugi geometry is literal: settling on a crack pours gold along the fracture. The cleanest collapse in the bunch. |
| `revert` | **dissolved into `time` sub-glass** | `mirror time settle tick=N` | **Dissolution: `revert` is one operation on the time manifold.** Reverting is `time.settle` — write a new tick whose manifold matches tick N. The `time` glass exposes the full Elm-style time-travel surface: `focus` = look at state at tick N; `shift` = see from tick N (Elm scrub; no move); `split` = branch from that point; `settle` = the revert; `project` = filter history by predicate. **Forward-references `@epistemologic/reality/time` (Track G, open).** |
| `time` | **sub-glass** | `mirror time { focus, project, split, shift, settle }` | Elm-style time travel as a five-op glass over the tick manifold. `focus tick=N` = look at state at tick N (read past). `shift tick=N` = see FROM tick N without moving HEAD (the Elm scrub). `split tick=N` = branch a parallel timeline from that point. `settle tick=N` = write a new tick whose manifold matches tick N (this IS revert). `project predicate` = filter history by predicate. **Substrate composition map (Seam, 2026-06-05):** the 5-op surface condenses `boot/std/time.mirror`'s 9 actions: `focus`←`enter`; `shift`←`browse`+`step`; `split`←`fork`; `settle`←`restore` (composed across the ref-set of tick N); `project`←`timeline.snapshots` filtered by predicate. The substrate already carries the algebra; full `@epistemologic/reality/time` (proof-block shape + crack-interaction semantics) is **Track G, deferred per LRM**. **Sings** — time-as-substrate makes the five-op recursion natural; the manifold has graph structure (ticks → ticks), the five ops fit without forcing. |

### 2.2 The condensed top-level surface

```
# Five top-level operations on the PROJECT MANIFOLD
mirror focus    [target...]
mirror project  [predicate]
mirror split    [edge]
mirror shift    [functor]
mirror settle   [optic]

# Seven sub-glasses, each with the same five operations
mirror compile    [focus|project|split|shift|settle] [args]
mirror kintsugi   [focus|project|split|shift|settle] [args]
mirror shatter    [focus|project|split|shift|settle] [args]
mirror bootstrap  [focus|project|split|shift|settle] [args]
mirror join       [@peer] [focus|project|split|shift|settle] [args]
mirror reflect    [focus|project|split|shift|settle] [args]
mirror time       [focus|project|split|shift|settle] [args]
mirror crack      [focus|project|split|shift|settle] [args] [--open|--force]
```

That is **the entire surface**. Five top-level ops. Seven sub-glasses
(`compile`, `kintsugi`, `shatter`, `bootstrap`, `join`, `reflect`, `time`,
`crack` — eight names, seven plus `crack`). Every sub-glass has the same
five ops. **Same algebra everywhere.** A user who learns the five
operations once knows every command at every depth.

---

## 3. The on-disk file structure

The path-namespace property (B5) makes the directory layout literal:

**Note (Seam pass, 2026-06-05):** the `shards/mirror/cli/` subdirectory
below is a **forward promise** as of `f0af9e4`. `shards/mirror/cli.mirror`
exists (C1); the eight sub-glass shards listed below do not yet. §8
acknowledges this explicitly ("no new shards in this round"); this section
describes the **target** layout that the next round mints.

```
shards/mirror/
├── cli.mirror                  # prism @mirror/cli — the top-level five ops
└── cli/
    ├── compile.mirror          # glass @mirror/cli/compile  { f,p,s,sh,se }
    ├── kintsugi.mirror         # glass @mirror/cli/kintsugi { f,p,s,sh,se }
    ├── shatter.mirror          # glass @mirror/cli/shatter  { f,p,s,sh,se }
    ├── bootstrap.mirror        # glass @mirror/cli/bootstrap{ f,p,s,sh,se }
    ├── join.mirror             # glass @mirror/cli/join     { f,p,s,sh,se }
    ├── reflect.mirror          # glass @mirror/cli/reflect  { f,p,s,sh,se }
    ├── time.mirror             # glass @mirror/cli/time     { f,p,s,sh,se }
    └── crack.mirror            # glass @mirror/cli/crack    { f,p,s,sh,se }
                                #   (open/force/seal collapse to settle + flag)
```

**Eight `.mirror` files** declare the entire CLI surface (one fewer than
the pre-rename count would suggest: `revert` dissolved into `time` while
`time` was added, net same). `ls shards/mirror/cli/` IS the help text —
every file is a glass, every glass has the same five operations. The CLI
is **self-describing by directory listing**.

**The renames track the substrate vocabulary already on disk.** `crack`
↔ `\` (fracture, per `gap-tension-tensor-substrate.md`). `time` ↔
`@epistemologic/reality/time` (Track G). `join` ↔ Glanville's Conversation
Theory verb. The CLI altitude finally speaks the substrate's own vocabulary.

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
shards/mirror/cli/join/
  reed.mirror              # glass @mirror/cli/join/reed
  alex.mirror              # glass @mirror/cli/join/alex
```

…would make each peer a sub-glass. **This is NOT proposed for v0.1.**
Peers are dynamic; baking them into the on-disk structure is wrong. Instead,
`mirror join @reed` passes `@reed` as an argument to the
`@mirror/cli/join` glass's `focus` op. Depth-2 directories are
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

The third-state lifecycle `cracks → open → force → seal` collapsed
beautifully into the `crack` glass with the same five operations:

- `crack focus`   = `cracks` (list)
- `crack settle`  = `seal-with-gold`  (the natural settlement)
- `crack settle --open`  = `open`  (declare a new `\`)
- `crack settle --force` = `force` (`\!`, accept-loss-and-fill)

But this **flattens three distinct events onto one verb-with-flags**.
The cybernetic CLI's `open / force / seal` names were better at making
the lifecycle visible at the shell. The condensation is structurally
correct (one write op per glass, distinguished by typed-lambda arg shape)
but **costs the user a bit of shell legibility**. `mirror seal X` was
direct; `mirror crack settle X` is one more word.

**The rename softens this strain.** With `hole`, the verb-with-flag form
felt arbitrary — `hole settle --open` reads as bookkeeping. With `crack`,
the kintsugi geometry makes the verb-with-flag form **literal**: settling
on a crack IS the gold pour; `--open` declares a new fracture; `--force`
accepts loss. The lifecycle isn't being hidden under flags — the flags
name the kintsugi acts. The constraint still costs a word; the substrate
vocabulary just makes the cost honest. Proof block still carries the
full lifecycle distinction.

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

### 5.1 `cracks/open/force/seal` → `crack` glass — the sharpest condensation

**4 verbs → 1 sub-glass × 1 small flag-set.** The lifecycle stays visible
(via `--open / --force`), the algebra is preserved exactly, the
substrate's third-state vocabulary stays first-class. **The rename from
`hole` to `crack` made this the sharpest example of substrate vocabulary
singing in the spec.** `\` produces fracture per gap-tension-tensor-substrate.md;
`crack` carries the kintsugi geometry literally. Settle on a crack IS
the gold pour. The proof block records the kintsugi act. **The constraint
earned its keep here once, no matter what else happens.**

### 5.2 `kintsugi` as a five-op glass

`focus / project / split / shift / settle` map onto the tournament loop
**without forcing**. `kintsugi.settle` IS one iteration. `kintsugi.focus`
IS peeking the next move. `kintsugi.split` IS enumerating candidate
fills. The tournament was already the same algebra at a smaller scale;
the recursion just **made the algebra visible at the shell**. The
substrate told us this was the right shape.

### 5.3 `time` as a five-op glass over the tick manifold

**Dissolution: `revert` was a single operation on the time manifold; the
`time` glass is the full Elm-style surface that contains it.** Time has
graph structure (ticks → ticks); the five ops fit cleanly and the
recursion becomes natural rather than forced.

- `time focus tick=N` = look at state at tick N.
- `time shift tick=N` = see FROM tick N without moving HEAD (the Elm scrub).
- `time split tick=N` = branch a parallel timeline from that point.
- `time settle tick=N` = write a new tick whose manifold matches tick N
  (this IS the revert).
- `time project predicate` = filter history by predicate.

The eigenboard's proof block on `settle` records the cost of the
time-write. **The revert strain didn't relocate; it dissolved.** What was
one isolated five-op surface (revert-as-glass) became one operation
(`time settle`) inside a glass whose other four ops were already
structurally present in the time manifold but not exposed. The constraint
revealed there was more time-algebra available than the cybernetic CLI
had named. **Forward-reference: `@epistemologic/reality/time` (Track G,
open)** — the substrate already has the 9 actions in
`boot/std/time.mirror` (`enter`, `restore`, `browse`, `compare`, `replay`,
`fork`, `step`, `present`, `convert`) plus `type tick = monotonic`; the
5-op CLI glass condenses these (composition map in §2.1). The CLI glass
can land now; Track G fills in only the proof-block shape and the
crack-interaction semantics.

### 5.4 `join` with peer-as-path-argument

**Rename: `converse` → `join`.** `mirror join @reed` reading the peer as
the glass's argument feels **right** in the way `cd @reed` would feel
right in a graph filesystem. The constraint guided us toward "peers are
values flowing through the algebra," not "peers are sub-substrates."
That's a meaningful piece of architecture clarified by the recursion.

**Why `join` over `converse`:** Join names the **entry**; conversing is
what happens **inside**. Glanville's Conversation Theory points at the
verb of entry, not the activity. Peer-conversations stop being separate
verbs and become operations under join: `mirror join @peer` = settle
(enter), `mirror join focus @peer` = observe peer's eigenboard without
entering, `mirror join shift @peer` = see from their altitude. The CLI
verb names the doorway; the activity inside the room stays nameless
(which is correct — the activity inside is the user's, not the CLI's).

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

The renames sharpen this further: `crack`, `time`, and `join` are
substrate-native (`\` fracture, `@epistemologic/reality/time`, Glanville's
verb). The CLI's seven sub-glass names are no longer arbitrary porcelain
labels — they're **the substrate's own vocabulary made visible at the
shell**. Names-to-learn isn't just smaller; it's also the same vocabulary
the user encounters everywhere else in the system.

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
| `seal` (cheap to type)   | `mirror seal X`               | `mirror crack settle X`      |
| `cracks` (read-heavy)    | `mirror holes`                | `mirror crack focus`         |
| `watch` (modal)          | `mirror watch`                | `mirror focus --watch`       |
| `compile` (canonical)    | `mirror compile T`            | `mirror compile settle T` ⁕  |
| `revert` (time-travel)   | `mirror revert N`             | `mirror time settle tick=N`  |
| `join` (peer-entry)      | `mirror converse @reed`       | `mirror join @reed`          |
| Substrate carries it     | partial (porcelain in Rust)   | **whole CLI in shards/**     |

⁕ With `mirror compile T` aliasing to `compile settle T` via the glass's
default-op rule (bare invocation = focus, BUT `compile` is the named loop
so it can declare `settle` as its default for the action-form). See §7.

**Net:** 5 fewer names to learn (and the names that remain are
**substrate-native**: `crack` = `\` fracture, `time` = `@epistemologic/reality/time`,
`join` = Glanville's verb). ~20 more dispatchable leaves (because every
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
short for `mirror kintsugi focus`. Every glass declares its own default in
its shard.

**Exception (narrow, per-glass): canonical safe-write defaults.** A glass
declares `default settle` ONLY when its bare-form act is a canonical,
side-effect-bounded write the user expects from typing just the glass
name. In v0.1, this is **`compile` and `shatter` alone**: `mirror compile T`
runs the build; `mirror shatter T` materializes the `.shatter` projection.
Both are "produce an output artifact" acts that don't escalate beyond
the artifact.

**Why not `bootstrap`, `reflect`, `join` (also verb-named)?** Each has a
`settle` semantic too pointed for an implicit default (Seam pass,
2026-06-05):

- `bootstrap settle` = advance a phase. Real side effect on substrate
  state; the user wants to see status (`focus`) before advancing.
- `reflect settle` = land a correction to `eigenboard.mirror`.
  Algedonic-triggered only (per §2.1); cannot be the bare default.
- `join settle` = enter (commits an utterance to the transcript).
  Without an utterance, the bare form must read first (`focus` = observe
  peer's eigenboard).

The corrected rule: **`default focus` unless the glass explicitly
declares otherwise.** v0.1 declarations: `compile`, `shatter` →
`default settle`. Everything else (`kintsugi`, `bootstrap`, `join`,
`reflect`, `time`, `crack`) → `default focus`.

The earlier "action-named glasses default to settle" framing was a
mechanical rule that didn't survive contact with the per-glass semantics;
Seam's adversarial pass surfaced the strain. Each glass declares its own
default; the convention is a hint, not a law. (Note: `time` is a noun,
so the strain didn't surface for that glass; the rule's failure was on
the verb-named-but-write-pointed glasses.)

**Rule declared in the glass shard itself** via an explicit `default`
field (per `shards/mirror/cli.mirror` C1's `default(name, t, value)`
pattern):

```mirror
glass @mirror/cli/compile {
  default settle              # canonical safe-write — bare form builds
  focus    target
  project  predicate
  split    candidate
  shift    altitude
  settle   target
}

glass @mirror/cli/kintsugi {
  default focus               # bare form reads the next tournament move
  ...
}

glass @mirror/cli/reflect {
  default focus               # bare form reads eigenboard; settle is
                              # algedonic-triggered, not implicit
  ...
}
```

This is the **only convention** the recursion needs beyond the constraint.
Everything else falls out of the file structure.

---

## 8. What this spec does NOT propose

- **No new shards in this round.** This is exploration; ticks land later
  on `reed/shards-floor` or its successor.
- **No depth-2 directories.** `shards/mirror/cli/join/` is reserved
  but not minted. Peers are arguments, not substrate.
- **No `@epistemologic/reality/time` substrate work.** The `time` glass
  forward-references this substrate; the substrate itself is Track G
  (open). `boot/std/time.mirror` declares `type tick = monotonic`; that
  is sufficient for the glass to land. Full time substrate is deferred.
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

3. **`crack settle --open / --force` ergonomics (§4.2).** Is the
   verb-with-flag form acceptable, or do we relent and add `mirror crack
   open / crack force / crack seal` as direct ops? My read: keep the flag
   form; the rename from `hole` to `crack` already softened this strain
   (substrate vocab makes the flags name kintsugi acts); the lifecycle
   moves to the proof block where it belongs anyway.

4. **`reflect` vs top-level `focus` (§4.4).** Keep `reflect` or fold into
   `focus --meta`? My read: keep. The altitude distinction is structurally
   real and `focus --meta` undersells what reflect does (the write path
   to eigenboard.mirror is real and warranted).

5. **`watch` flag scope.** Does `--watch` work only on top-level `focus`,
   or on every glass's `focus`? My read: every glass. `mirror kintsugi
   focus --watch` should work the same as `mirror focus --watch`. The
   adverb composes with the verb at any altitude.

6. **`join @peer` argument shape.** Does `@peer` come BEFORE or AFTER
   the op? `mirror join @reed focus` or `mirror join focus @reed`?
   My read: BEFORE (path-shape). `mirror join @reed` walks toward the
   peer; the op is what you do once you're there. Join names the entry;
   the op names what you do at the doorway.

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

Where the recursion sang (`crack`, `kintsugi`, `time`, `join`,
dispatcher-as-path-walk), it **closed a loop** that mosaic opened at the
build altitude: the file structure IS the type, the type IS the
substrate, the substrate IS the algebra, the algebra IS the same five
operations at every altitude. **eⁿ⁺¹ ≤ eⁿ** at the CLI surface too — the
verb count goes down without the expressive surface going down. The
renames make the singing literal: `crack`, `time`, and `join` are the
substrate's own vocabulary, finally spoken at the shell.

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
