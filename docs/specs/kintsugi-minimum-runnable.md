# kintsugi-minimum-runnable — the engine performs the grammar→prism rename

*2026-05-29. Mara. Spec — design, not implementation. No Rust changes; one
markdown deliverable. The implementation tick sequence lives in §9 for a
following agent.*

Status: **Red** (the minimum-runnable structure is specified — fracture,
candidates, loss, application; the dispatcher question is decided; the
proof-of-engine checks are pinned. None of it runs yet. The implementation
ticks in §9 land it.)

Depends on:
- `docs/specs/prism-floor-and-the-grammar-rename.md` (commit `8fa145d`) —
  the nine-keyword floor, the gap audit, the rename's blast radius (158
  `.mirror` files use `grammar @x`, 9 ref `@mirror/grammar`, 3 `.rs` files
  hardcode the literal). **This spec supersedes its Tick 1.** Tick 1 of
  `8fa145d` proposed a mechanical `cmd_kintsugi --transform 'grammar => prism'`
  migration plus a coupled `.rs` change; that is sed-in-kintsugi-clothing.
  The audit (its §3, §4.2, §4.4) STANDS verbatim; only its Tick 1
  *mechanism* is replaced — by §1–§6 of this spec.
- `docs/specs/kintsugi-tournament.md` (commit `b1edb6c..`) — the bigger
  Red spec for the full multi-candidate tournament with Hajek-bounded
  convergence over a six-strategy closed sum type. This spec is the
  **first runnable slice** of that ambition: single-candidate, trivial
  tournament, real structure.
- `docs/specs/substrate-native-fate-tournament.md` (commit `c0bb724`) — the
  `au` coordinate and the tournament shape. Single-candidate fractures do
  not invoke the conductivity-Laplacian machinery; the structure remains
  compatible.
- `docs/specs/parse-as-fate-tournament.md` — §3.1 RESOLVED: the loss
  function at any altitude is the composite of `@epistemologic/properties`
  at that altitude. **Not** Shannon, **not** Dark count, **not** invented.
  This spec inherits the resolution verbatim.
- `docs/specs/kintsugi-formatter.md` — the five iteration stages
  (propose / measure / elect / verify / fixed-point). The dispatcher
  realises stages 1–4 trivially (single candidate); stage 5 is the OID
  equality the bootstrap already computes.
- `boot/std/kintsugi.mirror`, `boot/std/kintsugi/fracture.mirror`,
  `boot/std/kintsugi/fracture/{generic-brackets,refract-to-fixed}.mirror`
  — what `@kintsugi` substrate exists today. All action bodies are `\`
  (parked obligations). The structure to populate is present; the bodies
  to execute are not.
- `bootstrap/src/main.rs::{kintsugi_tick, cmd_kintsugi, cmd_kintsugi_migrate,
  cmd_kintsugi_single}` — `kintsugi_tick` is the explicit no-op scaffold
  ("every body is no-op"); `cmd_kintsugi --transform` is the tested
  whole-word byte rewrite + basename rewrite. Today they are not
  connected: the tick does not invoke the transform; the transform does
  not invoke the tick.
- `bootstrap/src/pipeline.rs::{parse_rewrite, apply_rewrites}` — the body
  of the rename's *application* step (whole-word, with `/` and `@` as
  boundaries; `grammar` does not match `grammars`). Tested.
- `bootstrap/src/grammar.rs` — the `grammar ` block opener (`grammar.rs:80`),
  `is_mirror()` ref equality on `"@mirror/grammar"`, the harvester. The
  coupled `.rs` change that moves alongside the substrate fracture.
- `boot/std/cli.mirror` — the existing `@cli` grammar (action bodies
  are inert; no dispatcher reads them). Cross-referenced by §2 because
  the dispatcher question intersects with `@cli`'s readiness.
- `docs/specs/mirror-new-command.md` (commit `c367512`) — names `@new`
  / `@cli` as substrate-declared commands whose bodies the bootstrap is
  supposed to dispatch. The same dispatcher question lands here.
- `boot/std/epistemologic/property/{coincidence_matches,total_classification,
  glass_wall}.mirror` — the three properties this spec composes into the
  kintsugi-altitude loss (§3).
- AGENTS.md §"Boundary Rust is not frozen capability" — the
  `[substrate-pull:realize]` carveout. The dispatcher harness, the
  block-opener rename, the renderer emit change all carry the marker.

Unblocks:
- The `grammar → prism` rename gets done — by kintsugi, not by sed under
  a fancy CLI flag. The rename becomes the **worked example** of
  minimum-runnable kintsugi rather than the chore that paved over the
  engine's absence.
- The `@kintsugi` substrate's parked bodies start to mean something. A
  `\` next to `fracture`, `enumerate`, `apply` becomes load-bearing once
  the dispatcher reads it.
- The `@cli` substrate gets a dispatcher too (if §2 picks framing (b))
  — the recurring "declare it as `@cli`, then invoke it from grammar"
  story (`mirror new` per `mirror-new-command.md`) becomes runnable
  alongside the kintsugi case. One harness, two consumers.
- A general path for multi-candidate fractures (`kintsugi-tournament.md`)
  — the structure this spec specifies is the same structure the full
  tournament populates non-trivially. Candidates becomes `[strategy]`;
  loss becomes the multi-property vector; application becomes a plan.
  The minimum-runnable engine is the seed.

---

## 0. The framing (locked)

The rename is the **simplest possible real kintsugi fracture**:

- ONE fracture declaration (`grammar => prism`)
- ONE candidate enumerated (the rename target)
- A loss measured from `@epistemologic/properties` (composite of three;
  §3)
- Application via the existing `cmd_kintsugi --transform` mechanism
  *invoked through* the fracture structure

The STRUCTURE must be real even if trivially populated. That is what
makes it kintsugi, not a byte rewrite invoked through a fancy command
name. Everything from `kintsugi-tournament.md` (multi-candidate,
Hajek-bounded convergence, the six-strategy closed sum type, the
conductivity-Laplacian scoring) is **explicitly out of scope here** — it
generalises the structure this spec lands, but does not need to land
with it.

The previous Tick 1 of `8fa145d` was honest about the engine being a
stub and proposed running the rename mechanically while the engine
matured. Alex's correction: *"I don't want the rename to be done
mechanically. I want us to close the gap so kintsugi can do the rename.
That's the whole point."* The rename is the **forcing function**.

The binary's gap is small. The engine's gap is the substrate-execution
dispatcher (§2). Closing the dispatcher closes the engine for trivial
cases. The rename then runs *through* the engine, and the engine exists.

---

## 1. The minimum-runnable structure — four pieces, real

The engine is a composition of four substrate declarations evaluated by
one Rust harness (§2). Each piece is real (it has a type, an OID, a
role in the pipeline); for the rename the piece is trivially populated.
For the multi-candidate case (`kintsugi-tournament.md`) the same pieces
are richly populated; the harness does not change shape.

Data flow:

```
  fracture declaration
       │
       │  fracture.enumerate(corpus) -> [candidate]
       ▼
  candidates : [candidate]    (single-candidate for the rename)
       │
       │  for each c: fracture.loss(c, corpus) -> [verdict]
       ▼
  loss profile : [(candidate, [verdict])]
       │
       │  fracture.elect(profile) -> result<candidate, no_winner>
       ▼
  winner : candidate           (trivial: the single one if loss passes)
       │
       │  fracture.apply(winner, corpus) -> imperfect(new_corpus, loss)
       ▼
  applied corpus
       │
       │  is_fixed_point(corpus, applied) by OID
       ▼
  done
```

Nothing else; nothing less. Each arrow is a substrate action whose body
the dispatcher evaluates.

### 1.1 Fracture — `fracture rename(old, new)` (the declaration)

A **fracture** declares a structural change the engine performs on a
corpus. For the rename:

```mirror
# boot/std/kintsugi/fracture/rename.mirror  (new — Tick A)
in @prism
in @kintsugi/fracture

grammar @kintsugi/fracture/rename {
  # Newtype the parameters. The rename targets the keyword altitude;
  # `old` and `new` are *keyword* values, not bare text. No-bare-types
  # discipline per `feedback-no-bare-types`.
  type keyword = text & boundary(word)
    # the boundary refinement enforces whole-word semantics: matching
    # respects non-word boundaries (the rule `apply_rewrites` already
    # encodes). A `keyword` value with internal whitespace or symbol
    # bytes is rejected at construction; only valid identifiers.

  # The fracture: rename one keyword to another, corpus-wide. The
  # fracture function is the closure operator `f(x) = rewrite(old=>new, x)`.
  # Its fixed points are corpora with no occurrences of `old` at
  # keyword-altitude positions.
  fracture rename(old: keyword, new: keyword) -> fracture { \ }

  # Enumerate candidate applications. For a rename the corpus is
  # surveyed once; the candidate list is `[rename_at(old, new, corpus)]`
  # — a single candidate. For a multi-keyword fracture (out of scope
  # here) the list would be one per keyword pair. The list is closed,
  # ordered, finite.
  enumerate(f: fracture, corpus: [path]) -> [candidate] { \ }

  # Apply one candidate to the corpus, producing the new corpus plus
  # the per-candidate loss. The application BODY is the bootstrap's
  # `apply_rewrites` (whole-word byte rewrite) + the basename
  # rewrite — see §4.
  apply(c: candidate, corpus: [path]) -> imperfect(corpus, loss) { \ }

  # Idempotence law — inherited from @kintsugi/fracture. Running the
  # rename twice produces the same corpus as running it once. The
  # second pass enumerates zero matches (every occurrence of `old` is
  # already `new`); the corpus is unchanged.
  requires idempotent(rename)

  # Canonical-at-fixpoint law — inherited. f(x) == x iff x is
  # canonical (i.e. contains no `old`-keyword occurrences at the
  # tracked altitude). Detection IS asking whether the fracture is a
  # no-op on the corpus.
  requires canonical_at_fixpoint(rename)
}

out keyword
out rename
out enumerate
out apply
```

The declaration *is* the fracture; the four actions plus the two laws
are its content. Each body is `\` — the dispatcher (§2) realises them.

### 1.2 Candidates — single, but enumerated through the structure

```mirror
# (extends @kintsugi/fracture/rename above)
type candidate = {
  fracture: ref,       # @kintsugi/fracture/rename/<oid>
  inputs:   { old: keyword, new: keyword },
  scope:    [path],    # which files the candidate touches
  oid:      oid,       # content-OID of the candidate itself, for replay
}
```

For `rename(grammar, prism)` over `boot/`, `enumerate` walks `boot/`,
groups occurrences by file, and returns:

```
[ candidate {
    fracture: @kintsugi/fracture/rename,
    inputs:   { old: "grammar", new: "prism" },
    scope:    [boot/std/mirror/grammar.mirror,
               boot/std/mirror/grammar/...,    # all 158 files
               boot/std/mirror/nl.mirror,
               ... (the 9 files referencing @mirror/grammar)],
    oid:      <content-hash of the above record>,
  }
]
```

Length one. The structure is `[candidate]` not `candidate`; the
multi-candidate case (e.g. `kintsugi-tournament.md`'s six strategies)
fills the list non-trivially.

### 1.3 Loss — composite of `@epistemologic/properties`

The loss for one candidate is **the verdict vector produced by reflecting
the `@epistemologic/properties` at kintsugi altitude over the post-
application state**. Verbatim from `parse-as-fate-tournament.md` §3.1:

> The loss function at any altitude is a composite of the
> `@epistemologic/properties` declared at that altitude. Not Shannon
> loss plus Dark coverage. Not a hand-invented metric.

For the rename, the kintsugi-altitude loss is the verdict triple:

| Property | Substrate ref | What it measures here |
|---|---|---|
| `coincidence_matches` | `boot/std/epistemologic/property/coincidence_matches.mirror` | the round-trip OIDs are preserved where expected. The renamed corpus's content-OIDs (under the renamed recognizer) match the pre-rename corpus's content-OIDs (under the pre-rename recognizer) for the *structure* of every AST. The rename is a relabel; structure must be invariant. |
| `total_classification` | `boot/std/epistemologic/property/total_classification.mirror` | no Dark spans are introduced post-rename. `count_dark(post) == count_dark(pre)` — the rename must not produce bytes the parser fails to structure. (A renamed file whose new keyword the post-rename parser does not recognize is exactly this kind of failure.) |
| `glass_wall` | `boot/std/epistemologic/property/glass_wall.mirror` | no non-mirror substrate crosses the `@io` boundary as a side effect. The rename touches `.mirror` files and the parser's recognition path; nothing under `@io` is touched, nothing outside `@io` becomes non-mirror. |

Composition is **the verdict vector itself** (per `parse-as-fate-tournament.md`
§3.1). The mapping verdict → axis contribution is fixed:

```
pass            → 0.0    (axis discharged)
partial(f, _)   → 1.0 - f
fail(_)         → 1.0    (axis saturated)
```

The **acceptance gate** for a single-candidate fracture is:

```
accept(candidate) <=>
  all verdicts in loss(candidate) are `pass`
```

This is the lexicographic-gates discipline of `kintsugi-tournament.md`
§3.2 collapsed to its trivial case: every property is a tier-1 gate;
any `fail` or `partial` eliminates the candidate. For single-candidate
fractures, elimination IS rejection of the whole fracture (no other
candidate to fall back to). The fracture is *not applied*; the corpus
is unchanged; the failure is reported with the failing verdict's
diagnostic.

Note what this is NOT: there is no Shannon loss, no Dark-count metric
directly, no hand-tuned weight, no Lagrangian. `total_classification`
includes the Dark count as part of its verdict; that is the *only*
place count_dark enters. No invention. No Shannon. The bootstrap's
existing `count_dark` (`bootstrap/src/main.rs::count_dark`) is the
implementation of `total_classification.dark_count` per the property's
own comment ("the bootstrap's `count_dark` function is the structural
sibling of this body") — already wired, already honest.

### 1.4 Application — invokes `--transform`, doesn't reinvent it

The existing `cmd_kintsugi --transform 'old => new'` is the right
**body** for a rename fracture's application step:

- `pipeline.rs::parse_rewrite` parses the rewrite query.
- `pipeline.rs::apply_rewrites` performs whole-word byte rewrite over
  the `.mirror` file's bytes; the boundary rules (`/` and `@` are
  boundaries; `is_word_byte` otherwise) are tested.
- `main.rs::cmd_kintsugi_migrate` walks `src_root` recursively, applies
  the rewrite, canonicalises destination paths (drops `std/mirror/` and
  `std/` prefixes), and performs the **basename rewrite** (e.g.
  `grammar.mirror` → `prism.mirror`).

These pieces are tested. The fracture engine does not reinvent them.
The engine INVOKES them through the fracture structure: when `apply` is
evaluated on the winning candidate, it dispatches to
`apply_rewrites` + the migrate walk as its body. The dispatcher (§2)
is what binds the `@kintsugi/fracture/rename.apply` action to the Rust
function.

The distinction matters: today's `cmd_kintsugi --transform 'grammar =>
prism'` is **the body** with **no engine wrapping it**. The rewrite
happens; no fracture is declared, no candidates enumerated, no loss
measured, no acceptance gate evaluated, no idempotence law checked.
That is what makes today's behaviour "sed-in-kintsugi-clothing". The
minimum-runnable engine wraps the same body in the fracture/candidate/
loss structure: same bytes get rewritten, but the *engine* did it,
because every step was a substrate action whose body the dispatcher
evaluated.

This is the precise definition of "kintsugi did the rename" used in
§6's verification: **the structured intermediate artifacts exist**.

---

## 2. The substrate-vs-floor split — the load-bearing decision

The minimum-runnable engine needs a Rust harness because the bootstrap
does not evaluate substrate action bodies today (§7.1). The question is
*which* harness. Two framings; recommend one.

### 2.1 Framing (a) — Specialised fracture dispatcher

Extend `kintsugi_tick` from its no-op scaffold to a minimum-runnable
harness scoped to `@kintsugi/fracture/*`. Concretely:

```rust
// bootstrap/src/main.rs — extended kintsugi_tick
fn kintsugi_tick(tick: u64, fracture_ref: &Ref, corpus: &Corpus)
    -> Result<Corpus, KintsugiError>
{
    let f = resolve_fracture(fracture_ref)?;    // load @kintsugi/fracture/<x>
    let cands = eval_enumerate(&f, corpus)?;     // run the substrate action
    let scored = cands.into_iter()
        .map(|c| (c.clone(), eval_loss(&c, corpus)))
        .collect();
    let winner = eval_elect(&scored)?;            // single-candidate: pass-or-reject
    let applied = eval_apply(&winner, corpus)?;   // invokes apply_rewrites
    let fixed = compute_content_oid(corpus) == compute_content_oid(&applied);
    log_tick(tick, &scored, &winner, fixed);
    Ok(applied)
}
```

Each `eval_*` reads the parked `\` body of the corresponding
`@kintsugi/fracture/<x>` action, looks up its Rust implementation in a
**kintsugi-only registry**, and dispatches. The registry is small,
static, and known at build time.

**Smaller scope.** Touches `main.rs` + a tiny `kintsugi_dispatch.rs`
module; no general substrate-execution machinery; no other consumer
needs to know.

**Cost.** The harness is special-purpose. The same architecture is
requested by `mirror-new-command.md` (`@cli` actions need a body
evaluator too) and will be requested by every future `@<x>` grammar
whose actions the bootstrap must run. (a) leaves a parallel harness
gap open.

### 2.2 Framing (b) — General substrate-execution dispatcher

The dispatcher reads parked `\` action bodies anywhere in `boot/std/`
and dispatches them to a static registry keyed by namespace + action
name (`@kintsugi/fracture/rename.enumerate` → Rust function ptr;
`@cli.compile` → Rust function ptr; etc.). The registry is the boundary
seam between substrate declarations and Rust implementations.

Concretely:

```rust
// bootstrap/src/dispatch.rs (new — Tick A)
// NOTE (amendment 2026-05-30): under the cascade,
//   `Registry` → `Crystallizations` (the plural of `Crystallization`),
//   `ActionPath` → `Ref`,
//   and `Splinter` becomes `Splinter<H: MerkleHash = Blake3>`.
// See §11 (amendment) and `docs/specs/store-vs-db-and-the-cascade.md`.
pub type ActionImpl = fn(&[Value]) -> Result<Value, Error>;
pub struct Registry { entries: BTreeMap<(Namespace, ActionName), ActionImpl> }

impl Registry {
    pub fn bootstrap() -> Self { /* build-time-known bindings */ }
    pub fn dispatch(&self, action: &ActionRef, args: &[Value])
        -> Result<Value, Error> { ... }
}
```

Two first consumers:

- `@kintsugi/fracture/rename.{enumerate,apply,elect}` (this spec).
- `@cli.{compile,run,craft,fate,kintsugi,serve}` (from
  `mirror-new-command.md` §5; the substrate has the actions, the
  dispatcher gives them bodies).

The dispatcher itself does NOT carry capability — it carries **binding**.
The capability stays in the substrate declaration and (for I/O effects)
in the `@io` boundary. The dispatcher is the floor that says *"this
substrate ref means this Rust function pointer"*. That is exactly the
shape AGENTS.md §"Boundary Rust is not frozen capability" describes:
"a grammar genuinely cannot describe a `write(2)` of itself" — and
equally, *a grammar cannot describe its own boundary binding to a
Rust function pointer*. The registry is that boundary.

**Bigger first slice.** Adds `dispatch.rs` (~150 lines), a `Value` type
for passing arguments, an error type, and the wiring from `cmd_kintsugi`
(and eventually `cmd_*` for `@cli`) into the registry.

**Cost.** More code lands in one tick. The Value type is a real design
decision (probably `enum Value { Path, Text, Keyword, Bytes, Verdict,
List(Vec<Value>), Record(BTreeMap<String, Value>) }` — small, closed,
extends with `@kintsugi/fracture`'s needs and `@cli`'s needs).

**Benefit.** ONE harness, TWO consumers. The `@cli` substrate-pull
request from `mirror-new-command.md` (which is otherwise its own tick)
lands alongside the kintsugi engine, because they want the same
dispatcher. The third consumer (whoever next needs to evaluate a
parked `\` body) reuses it for free.

### 2.3 Recommendation: framing (b)

Framing (b). Reasons in order of weight:

1. **Principled unification.** Two existing substrate-pull requests
   (`@kintsugi/fracture/*`'s bodies, `@cli.*`'s bodies) need the same
   boundary primitive: *evaluate a parked substrate action with the
   bound Rust implementation*. Framing (a) builds two parallel
   harnesses; (b) builds one. Per AGENTS.md, boundary Rust should
   surface the primitive the meta-grammar can't yet describe — not
   one per capability that needs it.

2. **The substrate already names the boundary.** Every `@kintsugi/*`
   and `@cli/*` action with a `\` body is implicitly saying *"my body
   is realised at the floor"*. Framing (b) makes that implicit naming
   structural: the registry IS the realization. Framing (a) leaves
   the implicit naming half-honoured (only kintsugi gets it).

3. **The registry is the right shape for marking.** A single
   `dispatch.rs` is the natural unit for the `[substrate-pull:realize]`
   marker — one floor primitive, one boundary file. Framing (a)'s
   special-purpose code blends into `main.rs` and is harder to audit
   for marker discipline.

4. **Multi-candidate generalisation is bounded.** When
   `kintsugi-tournament.md`'s full tournament lands, the additional
   substrate actions (`@kintsugi/merge.enumerate`, `.apply`, `.resolve`)
   register with the same dispatcher. Framing (a) would require
   extending `kintsugi_tick` again; (b) requires only new registry
   entries.

Framing (a) is honest about scope: it lands faster, it solves the
immediate problem. But it builds a special-purpose mechanism that will
be extended into a general one within the next two ticks (the `@cli`
body-evaluation tick is already on the horizon per
`mirror-new-command.md`). Slow-is-fast, no-workarounds (per
`feedback-slow-is-fast-no-workarounds`): build the general dispatcher
once. The first slice is bigger; the second through Nth consumer
costs nothing.

**Both framings honour the principle.** In neither does kintsugi LOGIC
live in Rust — only the boundary binding. Fracture detection, candidate
enumeration, loss composition, the elect rule all live in
`@kintsugi/fracture/*` substrate bodies. The harness invokes them; it
does not encode them.

### 2.4 What lives where, in framing (b)

| Layer | Lives in | Owner |
|---|---|---|
| Fracture declaration (`fracture rename(old, new)`) | substrate (`@kintsugi/fracture/rename.mirror`) | kintsugi grammar |
| Candidate type + enumeration body | substrate body (`enumerate`) | kintsugi grammar |
| Loss = `[verdict]` composition | substrate body (`loss`); the verdicts come from `@epistemologic/property/*.reflect(ast)` | property grammar |
| Acceptance gate (all `pass`) | substrate body (`elect`) | kintsugi grammar |
| Application body (byte rewrite + basename rewrite) | bootstrap (`apply_rewrites` + the migrate walk), bound through dispatcher | floor |
| OID equality (fixed-point check) | bootstrap (`compute_content_oid`), bound | floor |
| Dispatcher itself | bootstrap (`bootstrap/src/dispatch.rs`) | boundary, marked `[substrate-pull:realize]` |
| Block-opener `prism ` & renderer emit | bootstrap (`grammar.rs`, `spectral.rs`) | boundary, marked (the coupled .rs change) |

Kintsugi-logic-in-substrate is preserved. The dispatcher is the only
added boundary primitive, and it explicitly carries no capability.

---

## 3. The loss, composed — which `@epistemologic/properties`

For the rename fracture, the kintsugi-altitude loss is the verdict
vector from reflecting these three properties over the post-application
state. Each is selected for a specific failure mode the rename could
introduce:

### 3.1 `coincidence_matches` — round-trip OID invariance

**Substrate ref:** `@epistemologic/property/coincidence_matches`.

**What it pins:** the bootstrap's content-address (computed by
`@hash/coincidence`) is what the grammar predicts. For the rename, this
becomes the load-bearing relabel check:

```
For every file F in scope:
  oid_pre  = content_oid(tokenize(F_pre,  grammar_pre))
  oid_post = content_oid(tokenize(F_post, grammar_post))
  EXPECT  oid_post == relabel(oid_pre, old=grammar, new=prism)
```

Where `relabel` is the predictable transformation of the OID under the
keyword rename (the AST structure is the same; the keyword label
changes; the OID hash incorporates the relabel). The property succeeds
if every file's post-OID is the predictable transform of its pre-OID.

**What it catches:** a rename that accidentally changes AST STRUCTURE
(not just keyword labels) — for example, if `apply_rewrites` had a
boundary bug and rewrote `grammars` to `prisms`, the resulting file's
AST would differ structurally and the OID would diverge.

### 3.2 `total_classification` — no new Dark spans

**Substrate ref:** `@epistemologic/property/total_classification`.

**What it pins:** every byte of the source enters a recognized AST
node; `dark_count(ast) == 0` for fully classified corpora.

For the rename, the check is `dark_count(post) <= dark_count(pre)` for
every file in scope. The relabel must not introduce bytes the
parser fails to structure. The strict equality (== 0) is the goal; the
delta (`<= pre`) is the gate.

**What it catches:** the bootstrap-coupling failure. If the `.mirror`
bytes are rewritten `grammar` → `prism` but the `.rs` block-opener
still says `grammar `, every file in scope becomes Dark from the
parser's perspective. `total_classification.dark_count(post)` saturates;
the verdict is `fail`; the fracture is rejected. The `.rs` coupled
change (§5) is *forced* by this property — try to land the substrate
rename without the opener change and the property catches it.

**This is the load-bearing property.** It is the engine's structural
refusal to let a partial change land.

### 3.3 `glass_wall` — no `@io`-boundary leaks

**Substrate ref:** `@epistemologic/property/glass_wall`.

**What it pins:** every grammar in the post-application corpus is
either mirror-shaped (parses through `@mirror/grammar`, or rather
`@mirror/prism` after this rename) OR lives under `@io`. The disjunction
is structural and decidable.

For the rename, the check is mostly trivial: the `.mirror` files in
scope are already mirror-shaped, and the rename does not move them
under `@io`. The property serves as a **sanity gate** — it would
fail if the rename accidentally produced a file whose namespace was
neither mirror-tokenizable nor `@io`-prefixed (e.g. a bug introducing
`@gramamr/...` from a typo cascading through references).

**What it catches:** any rename that breaks the glass wall property of
the corpus. For this specific fracture, the check is near-trivial; it
is included because the loss is the **vector of relevant verdicts**,
not a hand-picked subset. The kintsugi-altitude properties include
`glass_wall`; the loss includes its verdict; if it would ever fail,
the fracture is rejected. (For future fractures that DO move substrate
across the wall — the cross-wall kintsugi cases that
`glass_wall.mirror`'s doc comment names — this same property becomes
the load-bearing gate.)

### 3.4 Composition — vector, not scalar

Per `parse-as-fate-tournament.md` §3.1:

> The composite is the vector of per-property contributions. Trajectory
> domination is component-wise on that vector. There is no separate
> "Shannon" axis and no separate "Dark" axis — if Shannon-style entropy
> or Dark-span coverage matter at parse altitude, they appear as
> properties under `@epistemologic/property/...` declared at that
> altitude.

The rename's loss is therefore:

```
loss(candidate) : [verdict] = [
  coincidence_matches.coincidence_matches(post_corpus),
  total_classification.total_classification(post_corpus),
  glass_wall.glass_wall(post_corpus),
]
```

No weighted sum. No scalar collapse. The acceptance gate is
*all three pass*. A single `partial` or `fail` rejects. For
multi-candidate fractures (later) the lexicographic ordering of
`kintsugi-tournament.md` §3.2 ranks accepted candidates by tail axes;
for single-candidate fractures, accept/reject is the whole verdict.

### 3.5 What the loss is NOT

- NOT `ShannonLoss` (the `prism` crate's metric, used for spectral
  beam decay). Wrong altitude. `ShannonLoss` is a beam-arithmetic
  primitive; the kintsugi loss is a property-verdict composite.
- NOT a hand-tuned weighted combination. The property set is the loss;
  the weights are fixed at 0/1−f/1.
- NOT the holonomy from `kintsugi-tournament.md` §3.1 C5. Holonomy is
  the tier-4 rank in the multi-candidate case; for single-candidate
  fractures there is no rank, only gates. Holonomy enters when multiple
  candidates pass the gates — out of scope here.
- NOT `count_dark` directly. `count_dark` is the *implementation* of
  `total_classification.dark_count`; it is wired through the property,
  not invoked separately.

---

## 4. The application body — what `--transform` already gives us

This section is **descriptive of existing code**. The point of this
spec is that the engine INVOKES the existing transform; it does not
replace it.

`bootstrap/src/pipeline.rs::apply_rewrites`:

- Iterates source bytes.
- For each rule `old => new`, scans for occurrences of `old`.
- Validates whole-word boundaries (`/` and `@` are boundaries,
  `is_word_byte` otherwise — so `@mirror/grammar`'s trailing path
  component rewrites; `grammars` does not).
- Emits the rewritten bytes.
- Sequential rule application (rule N sees rule N−1's output).
- Tested via `pipeline.rs` fixtures pinning the boundary cases.

`bootstrap/src/main.rs::cmd_kintsugi_migrate`:

- Walks a directory tree, collecting `.mirror` files.
- For each file: applies the parsed rewrite rules to its bytes.
- Canonicalises the destination path (drops `std/mirror/`, then `std/`).
- Performs the basename rewrite (e.g. `grammar.mirror` → `prism.mirror`).
- Writes the result.
- Tested by the existing rename fixtures.

This is the BODY of the application step. The minimum-runnable engine's
`@kintsugi/fracture/rename.apply` action dispatches to it through the
registry. The dispatcher passes the candidate's `scope` (the file list)
and `inputs` (the keyword pair); the bound Rust function does the
rewrite + basename + write.

The engine adds the structural wrapper:

```
[engine]                          [body]
enumerate(rename, boot/)   ─→     (walk corpus, group occurrences)
  ↓
[ candidate ]                     (single)
  ↓
loss(candidate, boot/)     ─→     (reflect properties, get [verdict])
  ↓
elect(scored)              ─→     (all-pass? winner : reject)
  ↓
apply(winner, boot/)       ─→     apply_rewrites + basename + write
  ↓
is_fixed_point(boot/, applied)  ─→     compute_content_oid equality
```

The body unchanged; the engine real.

---

## 5. The coupled boundary-Rust change

The `grammar → prism` rename touches three `.rs` files. These are NOT
what kintsugi changes — they are **boundary work that moves alongside
the substrate fracture in the same tick (or paired commits)**. From
`prism-floor-and-the-grammar-rename.md` §4.4 (which stands):

- `bootstrap/src/grammar.rs:80` — the block-opener literal `"grammar "`
  → `"prism "`.
- `bootstrap/src/grammar.rs:60` (and `spectral.rs:559`) — the ref equality
  `"@mirror/grammar"` → `"@mirror/prism"`.
- `bootstrap/src/spectral.rs:606` — the renderer emit `b"grammar "` →
  `b"prism "`.
- `bootstrap/src/spectral.rs:3020/3029` — combinator literals (per the
  existing audit).
- Test fixtures in `pipeline.rs` (not load-bearing for the rename, but
  the test names move).

These moves are `[substrate-pull:realize]`: the *capability* (open a
keyword block; emit it on render) is unchanged; only the trigger word
moves. Per AGENTS.md §"Boundary Rust is not frozen capability", that
is exactly the bypass-marker's purpose.

**The kintsugi engine does NOT perform the `.rs` change.** The engine
fractures `.mirror` substrate. The `.rs` opener/renderer move is
co-required because the parse breaks the moment the substrate side
lands without the opener side following. The two halves are paired:

- **Option 1 (single commit).** The `.rs` change and the substrate
  rename ride one commit. Mixed-content (Rust + .mirror) commit;
  carries `🔴/🟢 [substrate-pull:realize]`. The hook accepts the
  marker; the commit lands.
- **Option 2 (paired commits).** The `.rs` change lands first as
  `🔧 [substrate-pull:realize]` (boundary work, no test pair), then
  the substrate kintsugi-rename lands as `🔴/🟢` (the engine runs
  the fracture; the rename happens; the OID check passes). The build
  is red between the two commits — acceptable on a feature branch,
  not on main.

Recommendation: **Option 1**. The two halves are one structural
change (the parser's keyword and the substrate's keyword must agree at
every commit), and the hook discipline supports the marker.

The coupling is honest. The engine runs on substrate; the substrate's
relationship to its recognizer is a boundary; the boundary moves
through boundary Rust with the marker. No special-pleading.

---

## 6. Verification — the rename as proof of engine

How we know **kintsugi-the-engine** ran the rename (not `apply_rewrites`
directly with a fancy CLI wrapper). The verification is a battery of
checks; passing them all is the difference between "the engine exists"
and "the rewrite happened".

### 6.1 Structured intermediate artifacts exist

The engine produces *typed records* for each stage. They are:

- **The fracture declaration's OID.** `@kintsugi/fracture/rename`
  resolves to a content-OID via `@hash/coincidence`. Replaying the
  same fracture on the same corpus produces a byte-identical record
  (the fracture is its declaration; the declaration has an OID).
- **The candidate list.** A `[candidate]` value whose OID is
  deterministic in the corpus and the fracture inputs. For the rename,
  length one; its OID is computable.
- **The loss profile.** `[(candidate, [verdict])]` — typed; serialisable
  via the existing render path.
- **The settlement record.** A typed tuple
  `{ fracture: ref, winner: candidate, loss: [verdict],
  pre_oid: oid, post_oid: oid, tick: u64 }`. Written to
  `refs/kintsugi/<agent>/fractures/<oid>` (paralleling
  `kintsugi-tournament.md` §7.1's gestalt entry; same shape, simpler
  content for the trivial case).

The artifacts are not diffs. The engine's output is the typed record
plus the rewritten corpus. The record proves the engine ran the
stages; the corpus proves the application body ran.

### 6.2 The engine log shows fracture → candidates → loss → apply

The `kintsugi_tick` log line is not `"ran transform 'grammar => prism'"`.
It is:

```
tick 1
  fracture: @kintsugi/fracture/rename(old=grammar, new=prism)
  candidates: 1
    [0] rename_at(grammar->prism, scope=158 files)
  loss[0]: [coincidence_matches=pass, total_classification=pass,
           glass_wall=pass]
  elect: [0]                          ← accepted (all gates pass)
  apply: 167 files written            ← 158 .mirror + 9 ref-update files
  fixed_point: true                    ← second tick is a no-op (idempotence)
```

The log is the engine's transcript. Reading it makes the four-piece
structure visible. Compare to today's `cmd_kintsugi --transform`, which
logs only `wrote <path>` per file — no fracture, no candidates, no
verdicts, no settlement.

### 6.3 Round-trip OID stability

For a representative sample of the 158 `.mirror` files in scope, OIDs
are stable across the rename in the relabel-predictable way (§3.1).
Concretely:

```
For file F in {boot/std/mirror/{grammar.mirror, runtime.mirror,
              nl.mirror, ast.mirror, spec.mirror}}:
  oid_pre  = content_oid(tokenize(F, grammar_pre))
  oid_post = content_oid(tokenize(F_renamed, grammar_post))
  ASSERT  oid_post == relabel_oid(oid_pre, "grammar", "prism")
```

The rename is a relabel; structure is invariant. This is what the
`coincidence_matches` property gate checks at fracture-loss time; this
test is the same check at integration-test time, asserting on the
specific files.

### 6.4 cargo test passes

Existing tests + new tests:

- All existing `pipeline.rs` rewrite tests still pass (the body is
  unchanged).
- A new test invokes the engine on a small fixture corpus and asserts
  on the structured intermediate artifacts (the candidate list, the
  loss verdict, the settlement record).
- A new test asserts the engine log format.
- A new test asserts the OID stability of the rename on a fixture file.

### 6.5 Idempotence — re-running is a no-op

Running the rename a second time produces:

- The same fracture declaration (same OID).
- A candidate list of length **zero** (no `grammar`-keyword occurrences
  remain to rename).
- An empty loss (no candidate to score).
- `elect`: no winner (vacuous).
- `apply`: no work.
- `fixed_point: true` (corpus OID unchanged).

This IS the `idempotent` law of `@kintsugi/fracture`. The engine checks
it by construction (the second pass has nothing to do). The law makes
the engine *measurable*: if a second pass produces non-zero work, the
fracture is not a closure operator, and the engine has a bug.

### 6.6 A false fracture is rejected, not silently applied

Invoke the engine with `@kintsugi/fracture/rename(old=zorblax,
new=prism)`. Expected behaviour:

- `enumerate(rename, corpus)` returns `[]` (no `zorblax` keyword exists
  in the corpus).
- `elect([])` returns `no_winner`.
- `apply` is not invoked.
- The settlement record reports `no_winner(empty_candidate_list)`.
- The corpus is unchanged.

For a fracture that DOES find candidates but DOES fail the loss gate
(e.g. a deliberate fixture where the `.rs` block-opener is not updated
in lockstep, so `total_classification` fails), expected behaviour:

- Candidates non-empty.
- Loss verdict `[..., total_classification=fail(<diag>), ...]`.
- `elect`: rejected (gate failure).
- `apply` is not invoked.
- The corpus is unchanged.
- Exit code non-zero; the failure diagnostic names the failing
  property and the verdict's `diagnostic` payload.

The difference between "engine" and "sed" is that **the engine can
refuse**. Today's `cmd_kintsugi --transform` does not refuse; it
rewrites whatever bytes match. The minimum-runnable engine refuses
when the loss fails the gate. That is the engine being load-bearing.

### 6.7 The acceptance verdict for §9's implementing ticks

All six checks above pass on a clean tree. Then the engine has
performed the rename, the rename was through the engine, and the
structure is real.

---

## 7. Honest dependencies / out-of-scope

### 7.1 The bootstrap does NOT evaluate substrate action bodies today

Load-bearing finding. The `bootstrap/src/` tree:

```
ast.rs       grammar tree types
exec.rs      io_exec — shells subprocesses (the @io boundary primitive)
git.rs       git plumbing
grammar.rs   parser: harvest keywords, parse the grammar declarations
hash.rs      content-OID computation (CoincidenceHash<5,5>)
main.rs      cmd_compile, cmd_craft, cmd_kintsugi, kintsugi_tick (stub)
pipeline.rs  mq-query pipeline; parse_rewrite + apply_rewrites
spectral.rs  the render side; combinator emit; the round-trip half
tokenize.rs  the structural-form dispatch (io/match/select); Dark fallback
```

There is no evaluator file. No code reads a parked `\` body and
dispatches it to anything. `kintsugi_tick` is the closest: it has the
five-stage *outline* but each stage is the identity. The doc comment
is explicit: *"Every body is no-op for this scaffold."*

The `@kintsugi/fracture` substrate (`fracture.mirror`,
`fracture/generic-brackets.mirror`, `fracture/refract-to-fixed.mirror`)
declares `apply`, `idempotent`, `canonical_at_fixpoint`, `flow` — every
body is `\`. The substrate carries the declaration; the body waits for
a dispatcher.

**Implication for the dispatcher question (§2).** Framing (a) does
NOT trivially extend `kintsugi_tick` — there is no precedent for body
evaluation to extend from. Both framings build the dispatch primitive
from zero; the difference is scope (kintsugi-only vs general).
Framing (b)'s scope is bigger but is honest about the larger gap: the
bootstrap needs **a** body evaluator. Building one is the substrate-
pull. Framing (a) builds half of one inside `kintsugi_tick` and calls
it done; framing (b) builds the whole one as `dispatch.rs`. The work
is comparable; (b) reuses.

The finding does NOT change the spec's design. It does change the
weight on framing (b)'s recommendation: "extend `kintsugi_tick`" is
a misleading description in either case; the truthful description is
"add a dispatcher", and (b) is the cleaner shape of that addition.

### 7.2 The `@kintsugi` substrate is sparse but well-shaped

`@kintsugi`, `@kintsugi/fracture`, `@kintsugi/fracture/generic-brackets`,
`@kintsugi/fracture/refract-to-fixed`, `@kintsugi/lift`,
`@kintsugi/migrate`, `@kintsugi/shatter`, `@kintsugi/translate` — eight
grammars exist; all are declared shape with bodies parked at `\`.
This spec adds ONE more: `@kintsugi/fracture/rename`. The pattern is
the same as the two existing concrete fractures
(`generic-brackets`, `refract-to-fixed`): a single `flow` function,
the two `requires` clauses inheriting from `@kintsugi/fracture`. The
rename's `enumerate` + `apply` + `elect` are new (the existing
fractures didn't need them — they were single-pass closures), but
their shape is small.

Growing the substrate as part of the implementation is acceptable. The
`@kintsugi/fracture/rename.mirror` is a small file (§1.1 sketch is
~30 lines); it lands in Tick B of §9.

### 7.3 Single-candidate vs multi-candidate generalisation

For a keyword rename, the tournament is trivially one candidate. State
explicitly:

- Tournament shape collapses to single-candidate evaluation.
- Loss is measured (does the post-rename substrate satisfy the
  three properties?).
- If all verdicts are `pass`, apply; if any fails, reject.
- For multi-candidate (the full tournament of
  `kintsugi-tournament.md`), this generalises by:
  - `enumerate` returning a list of length > 1;
  - `loss` evaluated per candidate, producing `[(candidate, [verdict])]`;
  - `elect` running the lexicographic ranking of `kintsugi-tournament.md`
    §3.2 — gates first (the same property triple, plus the merge spec's
    `kintsugi_reachable`, `namespace_integrity`, `xref_integrity`), then
    rank by holonomy (C5), then tiebreak by OID churn (C4);
  - the strategy vocabulary (`@kintsugi/merge.strategy`) registered
    with the same dispatcher.

The minimum-runnable engine IS the seed of the full tournament. Nothing
changes in the harness when the candidate list goes from 1 to N; the
`elect` body becomes non-trivial. **The dispatcher recommendation (§2)
is exactly what supports this generalisation cheaply.**

### 7.4 `@fate` is NOT needed for single-candidate fractures

For a trivial tournament, the five Fate models don't propose anything
(only one strategy applies). `@fate` is needed for:

- Multi-candidate enumeration where the candidates are *creative
  proposals* (the five models suggesting merge strategies, per
  `kintsugi-tournament.md` §4.1).
- Au-coordinate scoring when properties alone don't rank.
- The holonomy computation (Magnot 2025) at tier 4.

None of these bite for the rename. `@fate.infer` is therefore a
**deferred dependency** of the full kintsugi tournament; it is NOT a
dependency of the minimum-runnable engine. The dispatcher can land,
the rename can run, the engine exists. Multi-candidate is a follow-on.

### 7.5 What's IN scope (this spec) vs OUT (the full tournament)

| Concern | IN this spec | OUT (deferred to `kintsugi-tournament.md`) |
|---|---|---|
| Fracture declaration shape | yes — `fracture rename` | yes — extended to merge strategies |
| Single-candidate enumeration | yes | no |
| Multi-candidate enumeration | no | yes |
| Loss = vector of property verdicts | yes — 3 properties at kintsugi altitude | yes — the same shape, plus C4/C5 ranks |
| Acceptance gate (all `pass`) | yes — collapses lex order to gate-only | yes — full tier 1–3 gates + tier 4/5 ranks |
| Hajek per-round convergence | no | yes |
| Conductivity tensor / `au` coordinate | no | yes |
| Six-strategy closed sum type (`@kintsugi/merge`) | no | yes |
| @fate.infer integration | no | yes |
| Holonomy computation (Magnot 2025) | no | yes |
| The dispatcher harness | yes — and reusable for `@cli` | reused |
| Application body (existing `apply_rewrites`) | yes — INVOKED through engine | reused |
| Coupled `.rs` block-opener change | yes — boundary, in same commit | no (no .rs change) |
| Round-trip OID verification | yes — §6.3 | yes — at the tournament level |
| Idempotence + false-fracture rejection | yes — §6.5, §6.6 | yes — same laws |
| Tournament gestalt entry (`refs/kintsugi/.../fractures/`) | yes — simpler shape | yes — richer shape (per §7.1 of tournament spec) |

The minimum-runnable engine is a strict subset of the full tournament's
capability. Nothing in this spec needs to be undone to get to the full
spec.

---

## 8. Relationship to existing specs

### 8.1 Supersedes `8fa145d`'s Tick 1

Explicit: **this spec supersedes Tick 1** of
`docs/specs/prism-floor-and-the-grammar-rename.md` (commit `8fa145d`).
That Tick 1 ("`grammar → prism` rename (the fracture, realized
mechanically)") proposed:

> `.mirror` (mechanical migration, `mirror kintsugi --out <tmp> --transform
> 'grammar => prism' boot/`): all 158 `grammar @x {` openers → `prism @x {`;
> all 9 `@mirror/grammar` → `@mirror/prism`; file rename ...
> Marker: 🔴/🟢 + `[substrate-pull:realize]` (the .rs recognition change is
> boundary Rust; red-first against a round-trip OID fixture).

What changes: the mechanical-migration step is replaced by an engine
invocation. Same files get rewritten; same `.rs` change happens; same
OID fixture is the verification. **But the rewrite is performed
through the engine** (fracture/candidate/loss/application), not
directly via `--transform`. The `[substrate-pull:realize]` marker still
applies (the coupled `.rs` change is unchanged), and additionally
applies to the dispatcher itself (§2.4).

The `8fa145d` audit (its §0 finding, §3 keyword table, §4.2 feasibility
verdict, §4.4 blast radius) **stands verbatim**. Only its proposed
*mechanism* (Tick 1) is replaced.

### 8.2 Should `8fa145d` get an amendment?

Recommendation: **a small 📝 amendment to `8fa145d`'s Tick 1 section**
that names this spec as the superseding mechanism. One paragraph,
at the head of §5's Tick 1 entry, reading approximately:

> **Superseded 2026-05-29 by `kintsugi-minimum-runnable.md`.** The
> mechanism below is replaced by the minimum-runnable engine: the
> rewrite is invoked through the fracture/candidate/loss structure,
> not through `cmd_kintsugi --transform` directly. The scope, blast
> radius (§4.4), and coupled `.rs` change requirement (§4.3) are
> unchanged.

Alternative: rely on this spec's §8.1 cross-reference and leave
`8fa145d` as the historical record. **My read: the amendment is
worth a 📝 commit** — `8fa145d` is a load-bearing audit and a future
reader following its Tick 1 link would otherwise miss the supersession.
The audit content stays; only the mechanism gets a redirect.

### 8.3 Cross-references to `kintsugi-tournament.md`

This spec is the **first runnable slice** of `kintsugi-tournament.md`.
The full tournament inherits this spec's:

- Fracture declaration shape (`fracture <name>(...)`).
- Loss = vector of `@epistemologic/property` verdicts.
- Acceptance gates (collapsed to gate-only here; lex-extended there).
- Dispatcher harness (registry; one for all consumers).
- Application body invocation pattern (engine wraps, body executes).

The tournament adds:

- Multi-candidate enumeration via the six-strategy `@kintsugi/merge`.
- Hajek convergence over rounds.
- The conductivity-Laplacian scoring at C5.
- `@fate.infer` integration.
- Per-tournament gestalt entries with the richer shape (per §7.1
  of the tournament spec).

When the tournament lands, this spec's `fracture rename` becomes one
entry in a larger family of fractures registered with the same
dispatcher. The structural compatibility is by design.

### 8.4 Cross-references to `substrate-native-fate-tournament.md`

The `au` coordinate and the tournament shape from
`substrate-native-fate-tournament.md` describe the type of the
multi-candidate scoring axis. For single-candidate fractures, no `au`
is produced (no contest); the loss is the property-verdict vector and
nothing more. The dispatcher harness's `Value` type (§2.2) should
include `Au` as a variant so the multi-candidate case lands without
a harness extension.

### 8.5 Cross-reference to `mirror-new-command.md`

If §2's recommendation lands (framing (b), the general dispatcher),
the `@cli` substrate from `mirror-new-command.md` §5 becomes a
second consumer in the same tick — or the next. The `@cli.compile`,
`@cli.craft`, `@cli.kintsugi` actions register with the dispatcher
the same way `@kintsugi/fracture/rename.enumerate` does. The CLI's
substrate-pull request and the kintsugi engine share infrastructure.

This is the principled-unification argument of §2.3 made concrete:
two specs that have been quietly waiting for the same boundary
primitive land it together.

---

## 9. The ticks — implementation sequence, post-spec

Once this spec is approved, the following ticks implement it. Each
follows TDD (red-first) and carries the FROZEN marker discipline
faithfully.

### Tick A — the general dispatcher harness

- **Scope:** add `bootstrap/src/dispatch.rs` (~150 lines) with the
  `Value` enum, `ActionImpl` type, `Registry` struct + bootstrap-time
  population. Wire `kintsugi_tick` to read fracture refs from
  substrate and dispatch their action bodies through the registry.
  The registry starts with one entry: `@kintsugi/fracture/rename.apply`
  → the existing `apply_rewrites` + `cmd_kintsugi_migrate` body. The
  other action implementations (`enumerate`, `elect`) land as small
  bound Rust functions in the registry.
- **Marker:** 🔴/🟢 `[substrate-pull:realize]`. Red-first against a
  test that invokes the dispatcher on a fixture fracture and asserts
  on the candidate list shape.
- **Verification:** the dispatcher resolves a substrate action ref,
  finds its registry entry, dispatches with arguments, returns a typed
  result. Cargo test exercises the path end-to-end on a fixture.
- **Out:** does NOT yet perform the grammar→prism rename (no fracture
  declaration is bound). Tick A is the floor; the fracture rides on
  it in Tick C.

### Tick B — the `@kintsugi/fracture/rename` substrate

- **Scope:** add `boot/std/kintsugi/fracture/rename.mirror` (the
  declaration sketched in §1.1; ~30 lines). Pure substrate; matches
  the existing `@kintsugi/fracture/{generic-brackets,refract-to-fixed}`
  pattern. All bodies are `\`.
- **Marker:** 🟢 (substrate-only; no test pair needed at the
  declaration altitude — the *engine* test is Tick C).
- **Verification:** the file parses through the bootstrap; the
  declared actions show up in `load_grammar` output; the property
  `requires` clauses resolve to `@kintsugi/fracture.idempotent` /
  `.canonical_at_fixpoint`.
- **Dependencies:** none (pure substrate add).

### Tick C — apply the engine to grammar→prism (THE rename)

- **Scope:** invoke the engine (Tick A's dispatcher + Tick B's
  substrate) on `boot/`. The engine performs the rename. **In the
  same commit** (Option 1 of §5): the coupled `.rs` boundary change
  — `grammar.rs:80` block-opener, `grammar.rs:60` ref equality,
  `spectral.rs:559/606` ref + emit, `spectral.rs:3020/3029` combinator
  literals — lands alongside. The 158 `.mirror` files use `prism @x`;
  the 9 `@mirror/grammar` refs become `@mirror/prism`; the
  `boot/std/mirror/grammar.mirror` file becomes `prism.mirror`.
- **Marker:** 🔴/🟢 `[substrate-pull:realize]`. Red-first against
  the six verification checks of §6 (engine artifacts exist, log
  format, OID stability, cargo test, idempotence, false-fracture
  rejection).
- **Verification:** all six §6 checks pass. `cargo test` green.
  `mirror craft boot/` produces a crystal with no new Dark.
- **Dependencies:** Tick A + Tick B both landed.

### Tick D (📝, follow-on) — doc reconciliation

- **Scope:** the specs that use "grammar"-as-keyword terminology
  become "prism". The 9 substrate-ref specs that
  `prism-floor-and-the-grammar-rename.md` §6 lists are the priority
  set. Generic "grammar"-as-CS-term mentions can be left.
- **Marker:** 📝. Sequential 📝 commits, one per spec or per small
  group.
- **Verification:** grep for `@mirror/grammar` returns zero hits in
  the priority specs; `grammar @x` references in the priority specs
  become `prism @x`.
- **Dependencies:** Tick C landed.

### Tick E (optional, 📝) — the `8fa145d` amendment

- **Scope:** add a one-paragraph supersession note to
  `prism-floor-and-the-grammar-rename.md` §5's Tick 1 entry, naming
  this spec as the mechanism (per §8.2).
- **Marker:** 📝.
- **Verification:** the cross-reference resolves; future readers
  following the audit's Tick 1 link find this spec.
- **Dependencies:** none (independent edit).

### Tick F (DEFERRED, post-tournament) — second consumer: `@cli`

- **Scope:** register `@cli.{compile,run,craft,kintsugi,serve}` action
  bodies with the same dispatcher. The `mirror-new-command.md` spec's
  `@new` actions also register. The dispatcher becomes the universal
  body evaluator; the bootstrap's `cmd_*` functions become thin
  shells that dispatch through the registry.
- **Marker:** 🔴/🟢 `[substrate-pull:realize]` per family.
- **Verification:** `mirror new ...` works as the substrate declares.
- **Dependencies:** Tick A landed; `mirror-new-command.md`'s
  implementing ticks scheduled.

Ordering rationale: Tick A is foundational (the dispatcher). Tick B
is the substrate (pure add; could land independently). Tick C is the
payoff (the engine runs the rename; the rename is the verification).
Tick D is corpus hygiene. Tick E is bookkeeping. Tick F is the
principled-unification cash-in.

---

## 10. Honest risks

- **The dispatcher's `Value` type is a real design surface.** Getting
  it small and closed for kintsugi+`@cli` matters. Too narrow and the
  next consumer needs an extension; too wide and the harness becomes
  capability. The proposed shape (§2.2) is `Path / Text / Keyword /
  Bytes / Verdict / Au / List / Record`. If during Tick A a needed
  variant surfaces that doesn't fit this set, surface the gap; do
  not paper over it. The marker discipline protects the dispatcher
  from accidental capability growth.
- **Property reflection's body itself is `\`.**
  `@epistemologic/property.reflect(ast) -> [verdict]` has a parked
  body. For the rename, the three properties' verdict bodies are also
  `\` (but each names its bootstrap-side sibling — e.g.
  `total_classification.dark_count` is bound to `count_dark`). The
  dispatcher (§2) is what binds them. Tick A's registry must include
  the property verdict bindings (three small entries) for the loss
  evaluation to work. This is in scope for Tick A; not deferring.
- **OID stability under relabel.** The `coincidence_matches` check
  relies on `relabel_oid(oid_pre, old, new)` being computable — i.e.,
  on the OID being a predictable function of the AST's keyword-labels.
  `CoincidenceHash<5,5>` is content-addressed over the AST including
  keywords, so a keyword change DOES change the OID; the predictability
  is that the change is **deterministic** given the keyword pair. The
  test (§6.3) can compute both sides and compare; the property's
  verdict body computes the same comparison structurally. If the
  predictability turns out to be hard to express in substrate (the
  property's body would itself need to know `CoincidenceHash<5,5>`),
  collapse the check to: "oid_post != oid_pre AND the only differences
  in the AST are at keyword-label positions." Structural delta only;
  no OID arithmetic.
- **The dispatcher might want to BE the kintsugi engine.** It's
  important to keep the engine logic (the four-piece data flow) in
  the *substrate*, not the dispatcher. The dispatcher is
  declaration→implementation binding only. The flow
  `enumerate → loss → elect → apply → is_fixed_point` is composed in
  the substrate (likely a `@kintsugi.run(fracture, corpus)` action
  that calls the four-step body in order). Tick A must resist the
  temptation to encode the flow in Rust; it would be the wrong
  altitude. Tick B's `@kintsugi/fracture/rename.run` (or
  `@kintsugi.run` composing over `@kintsugi/fracture/*`) is where the
  flow lives.
- **Single-commit landing of substrate + .rs change carries a risk.**
  If anything in Tick C's verification fails post-commit, the
  combined change is harder to bisect. Mitigation: the red-first test
  pair in Tick C asserts all six §6 checks before either side is
  committed. The hook's `[substrate-pull:realize]` discipline does
  not waive the test-pair requirement; the 🔴 commit must precede
  the 🟢 one, with the test pair providing the safety net.

---

*The rename was a chore. The chore was the engine's absence in
disguise. The minimum-runnable engine is what closes the gap: four
real pieces, a single candidate, a vector of three verdicts, the
existing body invoked through the structure. When it lands, the
fracture happens, the engine exists, and the next fracture (multi-
candidate, full tournament) extends what is real rather than
inventing it. The proof is the rename — done, not by sed in fancy
clothes, but by the engine that the rename forced into existence.*

---

## 11. Amendment — cascade renames (2026-05-30)

Same day as commit `c3a01e3` (the SpectralHash design spec) and `e9c259b` (the
CHC collapse spec), Alex and Reed continued the architecture conversation and
LRM-collapsed several over-built proposals into a simpler shape. The four-piece
structure (fracture / candidates / loss / application), the substrate-vs-floor
split (§2), and the loss-from-`@epistemologic/properties` grounding (§3) **all
stand verbatim**. What changes is naming + the underlying type generic, and one
pinned next-tick item:

### 11.1 Renames (apply throughout the cascade)

- **`Registry` → `Crystallizations`.** The table that holds bound substrate
  action implementations is the plural of `Crystallization` (the singular event).
  Names the discipline; matches the kintsugi vocabulary. Affects §2.2's sketch
  (`pub struct Registry { ... }` → `pub struct Crystallizations<H> { ... }`),
  §2.4's "Dispatcher itself" row, Tick A's scope statement.
- **`ActionPath` → `Ref`.** Matches mirror's nav-ref vocabulary (the `.`, `..`,
  `...`, `~`, `@`, `^`, `HEAD` set). `action` is dead since we have prism / glass
  / 5-ops, not "actions". Affects every internal use; the substrate-side
  declaration syntax (`action enumerate { ... }`) is *not* part of this rename
  (that's the substrate's own surface).

### 11.2 Generic over hash backend

The Merkle tree becomes generic over the hash algorithm. Per Alex: *"If we make
the AST/MerkleTree generic over the hash algorithm... then everything else falls
out."* Cascade:

- `Splinter<H: MerkleHash = Blake3>`
- `Content<H>`
- `Body<H>`
- `Crystallization<H>`
- `Crystallizations<H>`
- `kintsugi_tick<H>`

Hash-blind types stay concrete: `Ref`, `CrystallizeError`, `IoError`, `ScalarLoss`
(the last is pinned for rename — see §11.4).

Default `H = Blake3` for `@mirror/store` (open content-addressed storage gate;
standard, fast, no float dependency, sidesteps Attack 1 from `spectral-hash-design.md`
§3.1). Other consumers (e.g. `@spectral/db`'s `VoidPointer` space) pick their own
primitive — `VoidPointer` is NOT a hash function but a spectral coordinate, so it
lives outside the `H` generic; see the new spec
`docs/specs/store-vs-db-and-the-cascade.md`.

A single bootstrap binary hosts multiple `H`-worlds. The dispatcher (Tick A) takes
the consumer's `Crystallizations<H>` rather than a single global table; storage
and engine consumers carry their own.

### 11.3 `@mirror/store` vs `@spectral/db`

The storage gate is `@mirror/store` (open foundation, content-addressed, git-backed
by default, where verification on write lives). `@spectral/db` is the engine on
top (potentially closed source, navigation / spectral graph). **mirror MUST work
without `@spectral/db`.** Open-foundation / closed-engine = both the business
model and the architecture. The kintsugi engine’s dispatcher belongs to the
foundation; it does not depend on the engine being present.

### 11.4 Pinned for next tick: `ScalarLoss` → `Transparency` (as a Lens)

The `[verdict]`-vector loss of §3 currently has a `ScalarLoss` aggregation at the
edges. After the cascade lands, `ScalarLoss` becomes **`Transparency`** as a Lens —
positive-frame (light passes vs absence-of-light), optical-family-native, natural
dual of Dark spans, lens-algebra-composable. **Not** baked in here; it is its own
tick. Noted so a future reader has the direction.

### 11.5 No structural changes

The four-piece data flow, the substrate-vs-floor decision, the `@epistemologic/properties`
grounding for loss, the dispatcher framing (b), the six verification checks of
§6, and the tick ordering (A → B → C → D → E, with F deferred) are **unchanged**.
The cascade is a naming + generic-parameter pass over the type tower; the
engine's data flow is the same.

### 11.6 Cross-references

- `docs/specs/store-vs-db-and-the-cascade.md` — landing page for the LRM-collapsed
  architecture.
- `docs/specs/spectral-hash-design.md` (commit `c3a01e3`, amended 2026-05-30) —
  upstream framing; recommendation rewritten same day.
- `docs/specs/coincidence-hash-collapse.md` (commit `e9c259b`) — SUPERSEDED top-
  banner.
