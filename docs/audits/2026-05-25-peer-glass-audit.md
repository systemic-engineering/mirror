# @peer glass — audit against the spec

*2026-05-25. Read-only audit. Scope: the seven open questions at the bottom of `docs/specs/peer-glass.md`.*

## Status digest

| # | Question | Status |
|---|---|---|
| 1 | `@peer` already exists as a boot grammar? | **missing** |
| 2 | Does it use `@cogito`? What does `@cogito` provide? | **exists (cogito); peer-side missing** |
| 3 | `~d` typed directory ref already a primitive? | **partial / misshapen** |
| 4 | `~mq` typed mq-query literal already a primitive? | **partial / misshapen** |
| 5 | Shape of `spawn` today (boot vs `@spectral/spawn`)? | **partial** (`@mirror/runtime/gen_prism.spawn` exists with the wrong signature) |
| 6 | Grammars in boot that already touch the five files? | **missing** (nothing references `identity.mirror`, `gestalt.mirror`, `tensions.mirror`, `eigenboard.spec`, `shatter.mirror`) |
| 7 | The gap = the implementation spec | see §3 below |

---

## 1. Does `@peer` already exist?

**Status:** missing.

**Evidence:** broad content search across `/Users/alexwolf/dev/projects/mirror/boot/**/*.mirror` for `@peer\b|grammar @peer` returns zero hits. No `boot/std/peer.mirror`. No `boot/std/peer/*.mirror`. The string `peer` only appears in two places in the boot tree, both prose:
- `docs/specs/mirror-store.md:777` — "spectral-db's storage layer is NOT a backend of mirror's store — it's a peer system that consumes the same fragmentation substrate."
- `docs/specs/mirror-store.md:965` — "They're peers."

**Gap:** The grammar must be authored from scratch. The target signature in `peer-glass.md` (`type peer = { identity, gestalt, tensions, eigenboard, shatter }`, `load(dir: ~d) -> peer`, `validate(p) -> imperfect`, `spawn(p, q: ~mq) -> gen_prism`) has no current home.

## 2. Does it use `@cogito`? What does `@cogito` provide?

**Status:** `@cogito` exists; `@peer` does not use it yet (because `@peer` doesn't exist), but it should.

**Evidence:** `boot/std/cogito.mirror` (1.9KB, 55 lines). Imports `@prism, @epistemologic, @epistemologic/math/bundle, @epistemologic/math/lawvere, @ai/fate, @mirror/execute, @beam`. Provides:
- `observe(imperfect) -> observation` (focuses holes; uses `@beam.emit`)
- `strategy(observation) -> tournament` (picks `elite(1).beam(8).halving(3)`)
- `perturb(observation, tournament_result) -> eigenboard` (writes the next eigenboard)
- `reflect(imperfect) -> imperfect = observe |> strategy |> perturb`
- `property autopoietic() -> verdict = @epistemologic/math/lawvere.is_autopoietic(@cogito)`

The head comment (`cogito.mirror:11-13`) explicitly notes that `perturb` writes "the eigenboard for the next tick" and that the concrete `type eigenboard` declaration "is a follow-up tick (probably at `boot/std/cogito/eigenboard.mirror`)". That file does not exist yet — `eigenboard` is currently an undeclared type.

**Gap:** A peer's `validate(p) -> imperfect` should be `p.tensions |> @cogito.reflect` (it is exactly the autopoietic-loop check the spec asks for). And the missing `type eigenboard` declaration is now load-bearing for `peer.eigenboard: spec`. The audit recommends materializing `boot/std/cogito/eigenboard.mirror` before `@peer.load` can type-check; otherwise `@peer` inherits the dangling type.

## 3. Is `~d` (typed directory ref) already a primitive?

**Status:** partial / misshapen.

**Evidence:** `boot/00a-sigil.mirror` declares the abstract sigil grammar:

```
abstract grammar @sigil(grammar, prefix: text) {
  type prefix = prefix | grammar
  type sigil(grammar, prefix, block(grammar))
  abstract template render(sigil) = iso
}
```

`boot/02a-io.mirror:15-18` instantiates three concrete sigils inside `@io < @sigil`:

```
type file(path) = @sigil("f")
type dir(path)  = @sigil("d")
type uri(path)  = @sigil("u")
```

Usage at the call site (`boot/std/mirror/compile/bootstrap.mirror`, ~20 occurrences) shows the *file* sigil with double-quote syntax — `@code/rust(~f"./bootstrap/src/hash.rs")`. The mq spec (`boot/std/code/mq.mirror:170`) documents the same: `@code/rust(~f"...") > fn[name="foo"]`.

No grammar uses `~d` in a call position anywhere in the boot tree. The type exists; the surface syntax is unexercised.

**Gap (two real issues):**
1. **Quote style.** The spec writes `~d'<dir>'` (single quotes); every existing site uses `~f"..."` (double quotes). One of the two has to give. Recommend matching the existing tokenizer (double quotes) unless `'..'` is intended to be a *typed* literal distinct from text — in which case the sigil grammar needs the second quote-form registered.
2. **Existence-check semantics.** The spec says `~d` "type-checks that the path exists and is a directory before the peer load runs." Today `dir(path) = @sigil("d")` is purely tag-on-text; no liveness check exists. This is a property obligation that needs to land in `@io` or in a new `@io/property/path_exists` grammar, and `@peer.load` must guard on it (or `~d` must guard on it at sigil-render time).

## 4. Is `~mq` already a primitive?

**Status:** partial / misshapen.

**Evidence:** `boot/std/code/mq.mirror` (8.1KB, 281 lines) declares `grammar @code/mq` with the full query AST (`focus_query`, `project_query`, `split_query`, `zoom_query`, `refract_query`, `intent_query`, plus context/suggestion/pattern machinery). It does NOT declare a `sigil("mq")`. There is no `~mq"..."` usage anywhere in the boot tree.

**Gap:** The sigil literal is unwired. To make `~mq'<query>'` real, one line is needed inside `grammar @code/mq` (paralleling `@io`'s file/dir sigils):

```
type mq_literal(text) = @sigil("mq")
```

…plus a `render` template that calls `@code/mq.parse` so the literal carries a parsed `query`, not just a text blob. Without this, `~mq` is a name the spec invented; the grammar exists, the sigil binding doesn't.

## 5. What's the current shape of `spawn`?

**Status:** partial. Two `spawn` surfaces exist; neither matches the target.

**Evidence (three sites):**
1. `boot/02a-io.mirror:23` — `io spawn(actor) => process` (abstract OS-level spawn).
2. `boot/02b-runtime.mirror:13` — `io spawn(triple: @shatter) -> pid` (abstract runtime spawn).
3. `boot/std/mirror/runtime/gen_prism.mirror:57` — the live one:

   ```
   spawn(name: text, initial_state: oid) -> gen_prism { \ }
   ```

`@spectral/spawn` does NOT exist in the boot tree. The spec docs (`mirror-runtime-gen-prism.md:252-258`) describe it as a *future* layer that "*inherits* from `@mirror/runtime/gen_prism`" and adds `think`/`decide` lambdas. No grammar file for it.

**Gap:** The target signature is

```
spawn : (@peer, ~mq) -> gen_prism
```

The nearest existing surface — `gen_prism.spawn(name: text, initial_state: oid)` — has the wrong arity, the wrong types, and no awareness of `@peer`. The right shape is for `@peer.spawn` to call through to `@mirror/runtime/gen_prism.spawn` after deriving `(name, initial_state)` from the peer's five files:

- `name` ← peer's `identity.mirror` ref (or its OID).
- `initial_state` ← crystallize the message body (the `~mq` query) into the initial state crystal.

This is mechanically simple but currently nowhere expressed.

## 6. What grammars in the boot touch the five files?

**Status:** missing across the board.

**Evidence:** content search for `identity\.mirror|shatter\.mirror|gestalt\.mirror|tensions\.mirror|eigenboard\.spec` across all boot `.mirror` files returns zero hits. Searches for bare `eigenboard\b` show seven boot files (`cogito.mirror`, `compose/weighted.mirror`, `epistemologic/math/bundle.mirror`, `lawvere.mirror`, `spectral-triple.mirror`, `code/mq.mirror`, `mcp.mirror`) — all use `eigenboard` as a *mathematical object* (a section of the principal G-bundle), never as a file extension. `shatter.mirror` only appears as the file name `boot/03-shatter.mirror` (and the grammar `@shatter` inside it) — never as a referenced path.

**Gap:** Nothing in the boot grammar knows that a peer's home is a directory of five named files. `@peer.load` will be the first grammar to assert this. The five names need a canonical declaration somewhere — most naturally in `@peer` itself:

```
type peer_file = identity | gestalt | tensions | eigenboard | shatter
fn filename(p: peer_file) -> text {
  identity   => "identity.mirror",
  gestalt    => "gestalt.mirror",
  tensions   => "tensions.mirror",
  eigenboard => "eigenboard.spec",
  shatter    => "shatter.mirror",
}
```

This is the only place where the filenames stop being magic strings and start being a typed enum the validator can iterate.

## 7. The gap = the implementation spec

See §3 ("The gap as implementation order") below.

---

## 3. The gap as implementation order

Dependency-ordered. Each item: file path → what to add → why.

1. **`boot/std/cogito/eigenboard.mirror`** (new) — declare `type eigenboard` as a section of the principal G-bundle on the five-operation graph (per `cogito.mirror:13-19` and `epistemologic/math/bundle.mirror`). Currently `eigenboard` is referenced but undeclared; `@peer.eigenboard: spec` will dangle without this. *Why first:* `@cogito.perturb` already returns `eigenboard`, so this fix is owed to the existing graph too.

2. **`boot/02a-io.mirror`** — add an existence-check property to `dir(path) = @sigil("d")`, OR add a sibling `~d_existing` sigil. *Why:* `peer-glass.md` requires `~d` to fail if the path is not a directory; today the sigil is pure tag-on-text.

3. **`boot/std/code/mq.mirror`** — add `type mq_literal(text) = @sigil("mq")` and a `render(sigil) -> query = parse(...)` template. *Why:* `~mq'<query>'` is not currently a recognized literal form. The full `@code/mq` query AST is here and ready to back it.

4. **`boot/std/mirror/runtime/gen_prism.mirror`** — no edit needed for v1; the existing `spawn(name: text, initial_state: oid)` is the right under-layer. Optionally add `spawn_from(state_crystal: oid) -> gen_prism` for the case where the name is derived from the state's identity field. *Why:* `@peer.spawn` will call through.

5. **`boot/std/peer.mirror`** (new — this is the spec target):
   ```mirror
   in @prism
   in @sigil
   in @io                          # for ~d
   in @code/mq                     # for ~mq
   in @cogito                      # for reflect
   in @mirror/runtime/gen_prism    # for spawn
   in @mirror/spectral             # for crystallize

   grammar @peer {
     type peer_file = identity | gestalt | tensions | eigenboard | shatter
     fn filename(p: peer_file) -> text { ... }

     type peer = {
       identity:   mirror,
       gestalt:    mirror,
       tensions:   mirror,
       eigenboard: spec,
       shatter:    mirror,
     }

     load(dir: ~d) -> peer { \ }
     validate(p: peer) -> imperfect { p.tensions |> @cogito.reflect }
     spawn(p: peer, q: ~mq) -> gen_prism {
       let name = @mirror/spectral.crystallize(p.identity);
       let init = @mirror/spectral.crystallize(q);
       @mirror/runtime/gen_prism.spawn(name, init)
     }
   }

   out peer out load out validate out spawn out @peer
   ```

6. **`boot/std/peer/property/manifold.mirror`** (new, optional) — encode the spec's manifold property: `gestalt` must be reachable from `identity` through valid Prism operations, `shatter` must be a valid ancestor chain. *Why:* the spec calls this out as a load-time obligation; deferring it to `validate` is a design choice (see Open Q below).

7. **`docs/specs/peer-glass.md`** — small edits: align quote style (`~d'..'` vs `~f".."`) and reconcile the `peer.eigenboard: spec` field type with the new `cogito/eigenboard.mirror` declaration. Mark **Status: Yellow → Green** when the above land.

---

## 4. Surprises

- **`@sigil` is already abstract and parameterized over a host grammar.** `boot/00a-sigil.mirror` says `abstract grammar @sigil(grammar, prefix: text)` — so `~mq'..'` is *already legal in principle*, only the concrete `@sigil("mq")` instantiation is missing. The substrate is more general than the spec assumed.
- **The five-axis Prism mapping is already half-encoded.** `compose/weighted.mirror`, `cogito.mirror`, `mq.mirror`, `mcp.mirror`, and the three `epistemologic/math/*` grammars all treat `eigenboard` as a first-class section. The peer spec's claim that `eigenboard.spec` is `zoom(self)` aligns with the existing literature — this is not a new invention, it's surfacing a contract that the rest of the graph has been silently assuming.
- **`@mirror/runtime/gen_prism.spawn` already has CAS-safe `send` and an ancestor-chain `history`.** Continuity (per the spec's identity-vs-continuity section) is essentially free — `history(gp, depth)` is already the "trajectory of the four mutable files across the ancestor chain." `@peer` doesn't need to reinvent this; it consumes it.
- **`@cogito` already carries the autopoietic-property check.** `property autopoietic() -> verdict` is exactly the manifold-on-its-own-trajectory check that `@peer.validate` wants. The implementation is one delegation: `validate(p) = @cogito.autopoietic` applied to the peer's tick map.
- **`@kintsugi.shatter` already does recursive fracture at Fiedler zero crossings.** `boot/std/kintsugi/shatter.mirror` is the right machinery for splitting a peer's `tensions.mirror` into bottom-up resolved pieces. The peer's `tensions: mirror` field can be a literal `@kintsugi/shatter` consumer.
- **`spawn` is overloaded three times in the boot.** `@io.spawn(actor)`, `@runtime.spawn(triple)`, and `@mirror/runtime/gen_prism.spawn(name, initial_state)` all coexist. The spec's `@peer.spawn(p, q)` will be a fourth. Worth a one-line note in `peer-glass.md` that this is intentional layering, not collision.
- **`@spectral/spawn` referenced everywhere, defined nowhere.** Two specs (`mirror-runtime-gen-prism.md`, `peer-glass.md`) treat it as an existing extension; the grammar file does not exist in the boot tree. `@peer` may end up being the de facto `@spectral/spawn` for v1.

---

## 5. Open design questions

Q1. **`validate` semantics: load-time vs first-tick.** The spec says "check that the peer is on its manifold" but doesn't say when. Option A: `load` runs `validate` and fails if invalid (strict — load is total). Option B: `load` is structural-only; `validate` is a separate call; `spawn` calls `validate` and fails if invalid (lazy — load is partial). Option C: `validate` runs continuously as a property over the `gen_prism` tick chain (Lawvere autopoietic check). Recommend B for v1, C as the property-layer follow-up.

Q2. **Sigil quote style: `'..'` vs `".."`.** Every existing site uses `~f"..."`; the spec writes `~d'...'`. Pick one. If the single-quote form is meant to denote a *typed* literal that the tokenizer guarantees has been validated (vs `".."` for raw text), that distinction should be promoted into `@sigil` as a second prefix kind. Otherwise unify on double quotes and amend the spec.

Q3. **`peer.identity` mutability.** The spec is clear that identity is immutable across the peer's lifetime; mutating it = fork. But `@mirror/runtime/gen_prism.send` allows any field of the state to change via the next tick. Does `@peer` need a custom `tick` that rejects identity-mutations, or is this enforced at the grammar level by making `identity` a `pure` type? Likely the latter (the meta grammar has `type pure = iso`), but the binding needs to be explicit.

Q4. **Five-file failure mode.** If `eigenboard.spec` is missing but the other four exist, `load` fails — fine. But what's the recovery? Does `@peer` offer `bootstrap(dir: ~d) -> peer` that materializes the missing four from `identity.mirror` alone (the manifold determines the initial section of the bundle)? This is the agent-onboarding story; the spec doesn't address it.

Q5. **`@spectral/spawn` vs `@peer.spawn`.** The spec says `@peer` does NOT replace `@spectral/spawn`. But `@spectral/spawn` doesn't exist yet, and the only documented difference is "heavyweight autonomous think loops." If `@peer.spawn` returns a `gen_prism`, and `gen_prism` already has the `tick` hook, what semantic surface does `@spectral/spawn` add that `@peer + @cogito.reflect + @fate/tournament` doesn't already cover? Worth resolving before either spec ships.

Q6. **`eigenboard.spec` extension.** Every other peer file is `.mirror`. `eigenboard.spec` breaks the pattern. Is `.spec` a real second grammar (the `mirror.spec` / `boot/07b-package-spec.mirror` family)? If yes, that grammar needs a one-line cross-reference; if no, drop the `.spec` and use `eigenboard.mirror` for orthogonality.
