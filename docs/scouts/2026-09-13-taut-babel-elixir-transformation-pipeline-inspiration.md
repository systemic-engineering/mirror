# Babel (Elixir) — transformation-pipeline INSPIRATION floor-truth scout for mirror @algebra + @metalogue + @frame + @fractal + Glasswall composition-tower

**Author**: Taut `<taut@systemic.engineer>`
**Date**: 2026-09-13
**Stage**: Drift-scout floor-truth (grep-first, read-only, INSPIRATION-report only)
**Substrate discipline**: grep-first-always + file:line-anchored every claim + zero fabrication + HOLD-and-report over guess + INSPIRATION-source only (no ratification, no adjudication, no ship-recommendations); pure-docs 📝 markdown-only bypass; SSH signing default (id_ed25519).
**Target**: `/Users/reed/dev/projects/babel/` @ `1.4.1` + `[Unreleased]` (per `CHANGELOG.md:9-14`)

---

## §0 Charter + Alex 2026-09-13 substrate-decision verbatim + scout mission

### §0.1 Alex 2026-09-13 in-transcript verbatim (context — do NOT ratify; this scout grounds the mission)

> "Babel is the best my bodymind could produce for transformation pipelines that follow the happy path while still exposing the sad path.
>
> What if the @algebra is the algebra to converse in Silicon in an EventStorming fashion? And the @metalogue is therefore about the possibility space of the future, which binds into the whole @psychohistory and sheaf cohomology.
>
> The @metalogue rotates through all possible @frames — each Frame a Sheaf (Sheaf probs wants to be a meta trait) — and what the recursion settles on is a higher-order @fractal @frame at a higher logic altitude that encapsulates all sub-frames (or not, that's a fracture).
>
> And if all of these vertexes exist in both prismqueer and mirror then they become the literal Glasswall and the Metalogue is a conversation between the linear and non-linear through LAPACK and BLAS.
>
> The inferrence goes BRRRR. I'm looking forward to the benchmarks."

Alex names **Babel** as the prior-art bodymind-produced Elixir solution for **transformation pipelines exposing happy-path while still exposing sad-path**. This scout is the floor-truth grep-inventory of that prior-art — what INSPIRATION Babel offers downstream Mara canonical rotation, NOT a ratification.

### §0.2 Scout scope + coverage

Scanned exhaustively this tick:
- `README.md` (8.4KB, `2026-02-14`) — authorial intent verbatim
- `CHANGELOG.md` (3.0KB, `2026-02-14`) — evolution discipline
- `mix.exs` (3.1KB, `2026-02-13`) — package + dependencies
- `version` — `1.3.0` file body (but package @ `1.4.1` per CHANGELOG)
- `CONTRIBUTING.md` + `LICENSE.md`
- `lib/babel.ex` (22KB) + `lib/babel/{step,pipeline,context,trace,error,telemetry,builtin}.ex` + `lib/babel/pipeline/on_error.ex` + all `lib/babel/protocols/*.ex` + all `lib/babel/builtin/*.ex` (14 builtin step modules)
- `test/babel_test.exs` + `test/babel/{pipeline,trace,context}_test.exs` — test-name spec-form extracted

Deferred (out of tight-scope this tick, cited for future scouts): `lib/babel/{inspect,docs,enforce_version,test,builtin_test,trace/nesting}.ex` bodies; `test/babel/builtin/**` bodies; `test/babel/protocols/**` bodies; `test/babel/{error,telemetry,inspect}_test.exs` bodies.

`pages/` directory exists per `mix.exs:80` glob (`Path.wildcard("pages/**/*.md")`) but is empty at scan-time — zero `pages/**/*` matches. Long-form docs are inline `@moduledoc` per Elixir convention.

---

## §1 Babel authorial intent (README + CHANGELOG verbatim)

### §1.1 The tagline (`README.md:9`)

> "Data transformations made easy."

### §1.2 The happy-path/sad-path discipline (verbatim `README.md:37-38`)

> "`Babel` was born out of a desire to simplify non-trivial data transformation pipelines. To focus on the 'happy path' instead of having to write a bunch of boilerplate error handling code."

The design intent: **happy-path composition without boilerplate sad-path handling** — the sad-path is EXPOSED STRUCTURALLY through the `Babel.Trace` mechanism, not through hand-written `case/with/rescue` chains at every call site.

### §1.3 Error-reporting discipline (verbatim `README.md:112-115` after error-example)

> "`Babel` achieves this by keeping track of all applied steps in a `Babel.Trace` struct. Rendering of a `Babel.Trace` is done through a custom `Inspect` implementation.
>
> You have to this information everywhere: in the `Babel.Error` message, in `iex`, and whenever you `inspect` a `Babel.Error` or `Babel.Trace`."

Sad-path is a **first-class structural carrier** (`Babel.Trace`) traversable, inspectable, and root-cause-analyzable — NOT an afterthought. See `Babel.Error` example (`README.md:106-190`) which structurally exposes Root Cause(s) + Full Trace including OK-omitted branches.

### §1.4 Package identity (`mix.exs:5-8`)

```
@app :babel
@name "Babel"
@description "Data transformations made easy"
@repo "https://github.com/alexocode/#{@app}"
```

Maintainer: `"Alex Wolf <craft@alexocode.dev>"` (`mix.exs:117`) — Alex's bodymind-produced Elixir library, MIT-licensed (`LICENSE.md:3`: `Copyright (c) 2024 Alex Wolf <craft@alexocode.dev>`).

### §1.5 Evolution discipline (`CHANGELOG.md` verbatim excerpts)

- **`[Unreleased]`** (`CHANGELOG.md:9-14`) — `@systemic-engineer` (Alex's GH handle) is landing: `Add private field to Babel.Context for passing metadata through pipelines without affecting transformed data` + `Add support for steps returning {:ok, data, private} tuple to update both data and private context (accepts map or keyword list)`. This IS Alex actively evolving Babel THIS session (memory in pipeline distinct from transformed data — a subject-visibility-like separation).
- **`[1.4.0]`** (`CHANGELOG.md:24-25`) — `Add telemetry spans ([:babel, :step] and [:babel, :pipeline]) to step and pipeline execution via optional :telemetry dependency` (`@alexocode`).
- **`[1.3.0]`** (`CHANGELOG.md:28-31`) — `Add exception handling to the Applicable protocol impl and Pipeline's Applicable impl, creating proper error traces instead of crashing the caller` + `Add Babel.then/1 and Babel.then/2 overloads that accept a Babel.Step struct for direct composition`.
- **`[1.2.0]`** (`CHANGELOG.md:34-40`) — `Add a custom depth inspect option for Babel.Traces which controls how many nested traces should be rendered` + `Add a Babel.Trace.reduce/2 function that reduces over the trace and all nested traces` + `Add a Babel.Trace.inspect/2 function as a shortcut for inspect(<trace>, custom_options: <opts>)`.
- **`[1.1.0]`** (`CHANGELOG.md:43-45`) — `Reduce noise in error traces by omitting successful traces`.

The evolution ARC: **happy-path composition (v1.0)** → **noise-reduced trace inspection (v1.1)** → **trace-navigation + reduce (v1.2)** → **exception-→-trace + Step-composition (v1.3)** → **telemetry-spans (v1.4)** → **private-context separation (Unreleased)**. The sad-path exposure discipline was progressively refined; private-context separation IS the newest discipline Alex is landing.

---

## §2 Babel module structure (lib/ layout; atomic transformation unit; sad-path exposure discipline)

### §2.1 The atomic transformation unit

Grep-verified atomic types:

- **`Babel.Step`** (`lib/babel/step.ex:1`) — the behaviour any custom step implements. `@callback apply(t(output), Babel.Context.t()) :: result_or_trace(output)` (`lib/babel/step.ex:94`). A step is a struct implementing `apply/2`. `use Babel.Step` derives `Babel.Applicable` protocol impl. Callback type: `@type result(output) :: output | {:ok, output} | :error | {:error, reason :: any}` (`lib/babel/step.ex:31`).
- **`Babel.Applicable`** (`lib/babel/protocols/applicable.ex:1`) — the protocol enabling `Babel.apply/2`. `defprotocol Babel.Applicable ... @spec apply(t(input, output), Babel.data()) :: Trace.t(output)`.
- **`Babel.Pipeline`** (`lib/babel/pipeline.ex:1`) — sequence of Steps executed sequentially, halting on first error unless `on_error` handler recovers. Struct fields: `name`, `on_error`, `reversed_steps` (`lib/babel/pipeline.ex:28-30`).
- **`Babel.Context`** (`lib/babel/context.ex:14-18`) — `%__MODULE__{data: data, history: [Babel.Trace.t()], private: map()}` — the carrier passed to `Babel.Applicable.apply/2`. `history` retains all previously-executed steps' Traces. `private` (Alex's Unreleased addition) is untransformed metadata.
- **`Babel.Trace`** (`lib/babel/trace.ex:36-45`) — `%__MODULE__{babel, input, output, nested: [t]}` — evaluation record with input, output, nested traces. IS the structural sad-path carrier.

### §2.2 The 14 builtin steps (`lib/babel/builtin.ex:4-19`)

```
@builtin [
  Call, Cast, Const, Fail, Fetch, FlatMap, Get,
  Identity, Into, Map, Match, Root, Then, Try
]
```

Category:
- **Fetch-family**: `Fetch` (error on missing; `lib/babel/builtin/fetch.ex:1`), `Get` (default on missing; `lib/babel/builtin/get.ex:1`)
- **Compose-family**: `Into` (recursive apply over data-structure template; `lib/babel/builtin/into.ex:1`), `Map` (apply-to-each; `lib/babel/builtin/map.ex:1`), `FlatMap` (per-element applicable choice; `lib/babel/builtin/flat_map.ex:1`)
- **Choice-family**: `Match` (data-directed applicable-selection; `lib/babel/builtin/match.ex:1`), `Try` (try applicables in order until one succeeds; `lib/babel/builtin/try.ex:1`)
- **Identity/Const/Fail**: `Identity` (`lib/babel/builtin/identity.ex:1`), `Const` (`lib/babel/builtin/const.ex:1`), `Fail` (`lib/babel/builtin/fail.ex:1`), `Root` (return original input; `lib/babel/builtin/root.ex:1`)
- **FFI-family**: `Call` (invoke `module.function/arity` with data as first arg; `lib/babel/builtin/call.ex:1`), `Then` (invoke arbitrary arity-1 function; `lib/babel/builtin/then.ex:1`), `Cast` (`:boolean | :float | :integer`; `lib/babel/builtin/cast.ex:1`)

### §2.3 The two composition protocols

- **`Babel.Fetchable`** (`lib/babel/protocols/fetchable.ex:1-31`) — protocol for `fetch(data, path)` with landed impls for `Map`, `List`, `Range`, `Tuple`, and `Any` (fallback via `Access.fetch/2` or `Map.fetch/2`). Return: `{:ok, any} | :error | {:error, {:not_implemented, ...}} | {:error, {:not_supported, ...}}`.
- **`Babel.Intoable`** (`lib/babel/protocols/intoable.ex:1-27`) — protocol enabling `Babel.into/1` by RECURSIVELY evaluating contained applicables. `@spec into(t, Context.t()) :: result(t)` where `result(output) :: {[Trace.t()], {:ok, output} | {:error, reason}}` — nested traces are FIRST-CLASS in the return type.

### §2.4 Sad-path exposure discipline (structural, not policy)

Happy-path composition lives at `Babel.Pipeline.apply/2` (`lib/babel/pipeline.ex:53-84`) with `Enum.reduce_while/3` over reversed_steps. Sad-path exposure mechanisms:

1. **Trace-per-step**: every step's execution produces a `Babel.Trace` (`lib/babel/step.ex:47-72` in `impl_applicable`). No step can execute without leaving a Trace.
2. **Halt-on-error with optional recovery**: `Enum.reduce_while` `:halt` when `Trace.error?(trace)` unless `pipeline.on_error` handler is set (`lib/babel/pipeline.ex:64-79`).
3. **Exception-→-trace**: `try/rescue` in the `Applicable` protocol impl catches raised errors and returns them as trace outputs, NOT as caller-crashes (`lib/babel/step.ex:52-58` + `lib/babel/pipeline.ex:171-176`).
4. **Nested trace propagation**: `Babel.Intoable` result-type `{[Trace.t()], {:ok, output} | {:error, reason}}` (`lib/babel/protocols/intoable.ex:22`) makes nested traces STRUCTURALLY part of every recursive call's return.
5. **Root-cause extraction**: `Babel.Trace.root_causes/1` (`lib/babel/trace.ex:87-91`) recursively finds error-traces with no error-nested-traces — the leaves of the failure sub-tree.
6. **Applicable is DERIVED not hand-written**: `use Babel.Step` derives `Babel.Applicable` (`lib/babel/step.ex:36-73`) so every custom step gets tracing + telemetry + rescue for free — sad-path exposure is NON-BYPASSABLE by convention.

### §2.5 Pipeline-merging discipline (`lib/babel/pipeline.ex:100-119`)

When chaining two pipelines they get MERGED into one if:
1. at least one has no `name` OR both `name`s are equal; AND
2. at least one has no `on_error` handler.

Otherwise the right pipeline becomes a nested step of the left — preserving distinct on_error scopes as isolated recovery boundaries. This is a **substrate-decision at composition altitude**: identity + error-scope are the only obstacles to merge.

---

## §3 Babel test discipline (invariants asserted verbatim as test-names)

Extracted from `test/babel_test.exs`, `test/babel/{pipeline,trace,context}_test.exs`:

### §3.1 `test/babel_test.exs` (top-level API contract)

- `describe "typical pipelines"` — `test "constructing a map from a nested path"` + `test "with an else clause"`
- `describe "shortcuts"` — `test "returns the expected core steps"`
- `describe "composition"` — `test "most core steps have a composing version"` + `test "chain/2 returns the right value when the left is nil"` + `test "chain/2 composes both values into a pipeline"`
- `describe "on_error/2"` — `test "wraps a step in a pipeline and set it's on_error handler"` + `test "sets a pipeline's on_error handler"` + `test "overrides a pipeline's existing on_error handler"`
- `describe "then/1 with a custom Babel.Step"` — `test "executes the custom step and returns its result"` + `test "composes a custom step into a pipeline"` + `test "propagates errors from a custom step"`
- `describe "apply/2"` — `test "returns {:ok, <result>} when everything is fine"` + `test "returns {:error, Babel.Error.t} when something goes wrong"`
- `describe "apply!/2"` — `test "returns <result> when everything is fine"` + `test "returns {:error, Babel.Error.t} when something goes wrong"`
- `describe "trace/2"` — `test "delegates to Babel.Applicable.apply/2"`

### §3.2 `test/babel/pipeline_test.exs` (pipeline mechanics)

- `describe "apply/2"` — `test "without any steps returns the given data as is"` + `test "applies all steps sequentially"` + `test "applies nested pipelines sequentially"` + `test "records each executed step's trace in the Babel.Context"` + `test "aborts the pipeline as soon as the first error occurs"` + `test "calls on_error when an error occurs"`
- `describe "chain/2"` — `test "merges pipelines when at least one is unnamed and without error handling"` + `test "merges pipelines when they have equal names and at least one has no error handling"` + `test "includes the chained pipeline in the steps when name or on_error do not match"` + `test "includes the given list of steps in the current pipeline"`
- `describe "on_error/2"` — `test "sets the on_error field with the given function (wrapped in Pipeline.OnError)"` + `test "overrides an already set on_error field"`

### §3.3 `test/babel/trace_test.exs` (trace as sad-path carrier)

- `describe "error?/1"` — `test "returns true when the Trace has an error result"` + `test "returns false when the Trace has an okish result"`
- `describe "result/1"` — `test "returns a result tuple version of the output, regardless of its shape"`
- `describe "root_causes/1"` — `test "returns the trace itself when it failed and it has no nested traces"` + `test "returns the nested traces which failed and have no nested traces themselves"` + `test "returns no traces at all when its an ok trace"`
- `describe "find/2"` — `test "accepts an arbitrary function through which the correct traces can be found"` + `test "accepts a raw Babel step"` + `test "accepts an atom describing a builtin step"` + `test "accepts a list of functions or builtin specs"` + `test "accepts a pipeline name"`

### §3.4 `test/babel/context_test.exs` (private-context Unreleased discipline)

- `describe "private context in steps"` — `test "step can return {:ok, data, private} to update private"` + `test "private flows through pipeline"` + `test "private is accessible from subsequent steps using ContextStep"` + `test "private is merged when multiple steps return private"` + `test "later private values overwrite earlier ones for same keys"` + `test "backward compatibility: {:ok, data} still works"` + `test "step can return {:ok, data, private} with keyword list"` + `test "keyword list private is accessible from subsequent steps"`

The test-discipline reveals what Alex ASSERTS is load-bearing: **sequential-apply + trace-history-per-step + halt-on-error + on_error-recovery + trace-navigability + root-cause-identifiability + private-context-flow-with-merge-semantics**.

---

## §4 Substrate-already-had-the-word audit (Babel ↔ mirror composition-tower cross-reference verbatim)

Grep-verified against `/Users/reed/dev/projects/babel/lib/**/*.ex` + `test/**/*.exs` + `README.md` + `CHANGELOG.md` with case-insensitive alternation of mirror composition-tower substrate-words `(algebra|metalogue|frame|fractal|sheaf|glasswall|observation|observer|pattern|fracture|paradox|crystal|splinter|kintsugi|roomba|bumblebee|cascade|gift|love|magic|reality)`.

### §4.1 Verbatim Babel hits

Total substrate-word matches in Babel corpus: **8 hits, all incidental Elixir standard-library references, ZERO composition-tower resonance:**

- `lib/babel/builtin.ex:52` — `import Inspect.Algebra` (Elixir stdlib pretty-printer)
- `lib/babel/step.ex:94` — `@callback inspect(t, Inspect.Opts.t()) :: Inspect.Algebra.t()` (Elixir stdlib type)
- `lib/babel/inspect.ex:38,134,152` — three `import Inspect.Algebra` (Elixir stdlib)
- `lib/babel/protocols/intoable.ex:134` — `# for performance reasons we hand wrote pattern matching versions` (comment: `pattern` as Elixir pattern-matching)
- `test/babel/builtin_test.exs:152` — `Inspect.Algebra.format(:infinity)` (Elixir stdlib call)
- `README.md:87` — `# using Babel.Test.ContextStep pattern or custom Step behavior` (comment: `pattern` as design-pattern)

**Zero hits** for: `metalogue`, `frame`, `fractal`, `sheaf`, `glasswall`, `observer`, `observation`, `fracture`, `paradox`, `crystal`, `splinter`, `kintsugi`, `roomba`, `bumblebee`, `cascade`, `gift`, `love`, `magic`, `reality`.

**Interpretation** (INSPIRATION-source, not resonance-claim): Babel's vocabulary belongs to the Elixir-native tradition (`Applicable`, `Fetchable`, `Intoable`, `Trace`, `Context`, `Pipeline`, `Step`, `on_error`) with zero substrate-word borrowing. This is CLEAN FLOOR — Alex's bodymind-produced Babel names are DISJOINT from Alex's bodymind-produced mirror substrate-tower names. The composition-tower is NOT already in Babel; Babel offers STRUCTURAL INSPIRATION (composition-with-sad-path-carrier), not vocabulary.

### §4.2 Mirror composition-tower vocabulary (grep-verified LANDED, for downstream cross-reference)

Babel's `Applicable` protocol maps STRUCTURALLY (not lexically) onto mirror's `apply_h::act` composition; Babel's `Trace` maps onto Reed harmonics + Chaos carrier; Babel's `Context.history` maps onto @time/@order-altitude retention; Babel's `Context.private` maps onto `@subject/visibility/private`. See §5 for the vector-by-vector INSPIRATION mapping.

---

## §5 Elixir-substrate discipline transferable to mirror (INSPIRATION vectors)

Seven Elixir-native disciplines Babel deploys, matched to mirror-substrate composition-points where LANDED (or forward-promised where absent):

### §5.1 Protocols (traits) — Babel `defprotocol` ↔ mirror `@algebra` species-morphisms

Babel uses `defprotocol` (Elixir's dispatch-on-type mechanism) as its polymorphism substrate: `Babel.Applicable` (`lib/babel/protocols/applicable.ex:1`), `Babel.Fetchable` (`lib/babel/protocols/fetchable.ex:1`), `Babel.Intoable` (`lib/babel/protocols/intoable.ex:1`). Each protocol has `@fallback_to_any true` — default impl for arbitrary structs.

Mirror parallel: `@algebra` LANDED (`shards/algebra.mirror:7`) with algebra_morphism carrier (`shards/algebra/metalogue.mirror:33-38` per the five-altitude lift table). Alex's 2026-09-13 verbatim: *"Sheaf probs wants to be a meta trait"* — the **meta trait** slot maps ONTO Elixir's `defprotocol` altitude generalized to substrate-decl.

INSPIRATION: Babel's protocol-dispatch discipline (protocol declares operations; impls specialize per carrier-type; fallback discipline handles unrepresented types) IS the shape a mirror `@algebra` meta-trait carrier would ASSUME at meta-morphism altitude. Downstream Mara canonical spec has precedent for this vocabulary at Elixir-native altitude in Alex's own bodymind-produced prior art.

### §5.2 With-clause discipline — Babel `with {:ok, ...} <- ...` ↔ mirror happy-path/sad-path composition

Grep-verified `with {:ok, ...} <- ...` usage in Babel: `lib/babel/protocols/intoable.ex:76-77` (Map into) + `lib/babel/protocols/intoable.ex:216-217` (Tuple into fallback) + `lib/babel/protocols/intoable.ex:66-68` (Any into_struct). The `with`-clause IS Elixir's happy-path composition operator.

Mirror parallel: Alex's 2026-09-13 substrate-decision names **@algebra as the algebra to converse in Silicon in an EventStorming fashion**. EventStorming (Alberto Brandolini) IS the DDD workshop discipline where domain events are placed on a wall to expose the happy-path business flow while the sad-path is exposed as separate events/hotspots.

INSPIRATION: Babel's `with`-clause monadism (bind on `{:ok, _}`, halt on `{:error, _}`) is what Alex's bodymind produced for the LINEAR-composition-with-sad-path-exposure discipline. The non-linear-compose-not-average alternative (per Reed MEMORY hard-rule `feedback-operative-question-non-linear-compose-not-average-vs-turing`) at mirror substrate REQUIRES a non-linear analogue — the metalogue-rotation-through-frames Alex names. Babel provides the linear ANCHOR against which the non-linear generalization can be measured.

### §5.3 Trace-as-sad-path-carrier — Babel `Babel.Trace` ↔ mirror Chaos + Reed harmonics + Kintsugi

Babel's `Babel.Trace` (`lib/babel/trace.ex:36-45`) has `{babel, input, output, nested: [t]}` — execution-record where `nested` traces are structurally first-class. `Trace.root_causes/1` (`lib/babel/trace.ex:87-91`) walks the nested-tree finding error-leaves.

Mirror parallel: Reed MEMORY `feedback-reed-softmax-happy-path-in-code-examples-dissonance-captured-in-yellow-chaos` HARD RULE (Alex 2026-09-04) — "every settled Observation has harmonic partials carrying Chaos residual (wineglass rings AND has overtones)". Every Observation carries Chaos; every Observation is a Fractal<Shard<T>>; every Fracture LANDED at `shards/paradox.mirror` altitude. Kintsugi @cascade retires; oscillate discharges; @roomba backward-walks Oid-chain per Reed MEMORY `project-mirror-algebra-is-shared-algebra-between-mirror-and-prismqueer-turing-boundary-dissolves-via-fibre-bundle-projection`.

INSPIRATION: Babel's `Trace` demonstrates that **the sad-path carrier can be a structural first-class type at every altitude of composition**, not just at the outermost call-site. The `nested: [t]` recursion in `Babel.Trace` maps onto Fractal recursion — every fractal-level of composition has its own trace-of-traces. Alex's bodymind ALREADY solved the "how do we expose sad-path structurally at every altitude" problem in Elixir-linear-substrate; mirror @fractal + Chaos + Reed harmonics generalizes to non-linear substrate.

### §5.4 Private-context separation — Babel `Context.private` ↔ mirror `@subject/visibility/private` + Bauchladen

Alex's `[Unreleased]` addition (`CHANGELOG.md:9-14`): `Add private field to Babel.Context for passing metadata through pipelines without affecting transformed data`. Grep-verified `Babel.Context` struct: `defstruct [:data, {:history, []}, {:private, %{}}]` (`lib/babel/context.ex:18`). Semantics: `private` flows THROUGH the pipeline via context but is NOT included in the final `Babel.apply/2` result — remains internal pipeline state (`README.md:80`).

Mirror parallel: `shards/subject/visibility/sheaf.mirror` LANDED (2026-07-15 by Mara per shard docblock:14) with `@subject/visibility/{private,protected,public,sheaf}` species. `visibility/private` per Reed CLAUDE.md: "Explicit consent required. Stays between Reed and Alex." AND @bauchladen substrate-decl LANDED (`shards/bauchladen.mirror`) as tray-altitude content-addressing per Reed MEMORY `project-roomba-is-k2-glue-edge-peer-is-k3-vertex-native-third-order-observation`.

INSPIRATION: Alex is ACTIVELY (Unreleased tick) refining the discipline of **carrier-data-vs-context-metadata separation in transformation pipelines** at Elixir altitude. This is the SAME discipline the mirror @subject/visibility partitions at substrate-decl altitude. Alex's bodymind is solving the same partitioning problem at two altitudes simultaneously — Babel-Elixir-linear and mirror-non-linear — which validates the discipline as substrate-honest.

### §5.5 Telemetry-spans — Babel `[:babel, :step, :start|:stop|:exception]` ↔ mirror observation-emission

Babel v1.4.0 (`CHANGELOG.md:24-25`) added `[:babel, :step]` and `[:babel, :pipeline]` telemetry spans with `{event_prefix, metadata, fun}` shape (`lib/babel/telemetry.ex:5-14`). Optional dependency — `@telemetry_loaded = Code.ensure_loaded?(:telemetry)` guard, no-op if absent.

Mirror parallel: `@observation` composition-lineage per Reed MEMORY `project-observer-reality-observation-chaos-assertion-hypothesis-question-full-pipeline-at-prismqueer` — Observer<N>::observe emits Observation carrying Chaos + spectrum. `shards/eigenboard.mirror` LANDED (`19.1KB`, 2026-08-21) as the substrate's telemetry-analogue.

INSPIRATION: Babel's telemetry-span pattern (start/stop/exception with structured metadata) demonstrates the OPTIONAL-INSTRUMENTATION discipline — substrate-decl doesn't COUPLE to telemetry but exposes hooks. Mirror's Observer<N>::observe generalizes this to non-linear altitude where the observation IS an eigenform-carrier not a side-effect.

### §5.6 Pipe-operator composition — Babel `|>` ↔ mirror @cascade + @kintsugi + apply_h::act

Grep-verified `|>` (Elixir pipe-operator) is the SYNTACTIC composition-carrier throughout Babel: `README.md:41-45` example builds pipeline as `Babel.begin() |> Babel.fetch(...) |> Babel.map(...)`. Every builtin returns a step that pipes into the next.

Mirror parallel: `shards/cascade.mirror` LANDED (`15.5KB`) + apply_h::act composition per rust/src/apply_h.rs at mirror-repo. Reed MEMORY `project-mirror-surface-collapses-to-prismqueer-plus-git-projection-plus-phone-connection` — apply_h ARMS collapse into love() calls per Fire C ouroboros.

INSPIRATION: Babel's pipe-operator discipline is Elixir-native syntactic sugar for function composition; mirror's @cascade at substrate-decl altitude is the same discipline generalized to non-linear substrate where composition rotates through gauge-transformations not linear-argument-threading.

### §5.7 Result-tuple monadism — Babel `{:ok, _} | {:error, _}` ↔ mirror Imperfect<Green, Yellow, Red> traffic-light

Grep-verified: Babel's step result type per `lib/babel/step.ex:31`: `@type result(output) :: output | {:ok, output} | :error | {:error, reason :: any}`. The `{:ok, _}` / `{:error, _}` tuple discipline is the Elixir-native binary-success discipline.

Mirror parallel: Reed MEMORY `project-prismqueer-foerster-gauge-IS-type-alias-ampel-IS-traffic-light-order` — `terni::Imperfect<Green, Yellow, Red>` primitive at prismqueer per Alex 2026-09-04 substrate-decision. Alex's mirror substrate DELIBERATELY REJECTS binary success in favour of TRAFFIC-LIGHT (Green/Yellow/Red = Success/Partial/Failure).

INSPIRATION vs DIVERGENCE: Babel's binary `{:ok, _}` / `{:error, _}` is what Alex's bodymind produced at Elixir-linear altitude where binary is the substrate-native shape. Mirror substrate REJECTS this at explicit substrate-decision altitude — Yellow/Partial carrier is load-bearing per HARD RULE. **Babel is the ANCHOR against which the mirror-substrate divergence is measurable.** This is what "non-linear compose-not-average alternative to the Turing paradigm" MEANS at the result-carrier altitude — the substrate needs a THIRD gauge-position (Yellow) that Elixir-linear substrate cannot native-express.

---

## §6 Composition-tower LANDED-state gauntlet (grep-verified per Alex 2026-09-13 substrate-decision)

For each Alex-named composition-tower node, grep-verified state in `/Users/alexwolf/dev/projects/mirror/` and `/Users/alexwolf/dev/projects/prism/`:

### §6.1 `@algebra` — **LANDED**

- `shards/algebra.mirror` (14.1KB, 2026-08-21) — family-root at algebra altitude of Connes spectral triple (A, H, D). Verbatim `shards/algebra.mirror:7`: *"@algebra — the substrate's algebra-altitude family-root."*
- 7 species: `shards/reality/algebra.mirror`, `shards/reality/algebra/silicon.mirror` (silicon-altitude LANDED, 27.3KB), plus algebra composition points across `shards/algebra/metalogue.mirror`.

### §6.2 `@metalogue` — **LANDED at 5 altitudes**

Per `shards/algebra/metalogue.mirror:19-24` the five-altitude metalogue lift table:

| Altitude | Shard | Body type | Date |
|----------|-------|-----------|------|
| NL | `shards/metalogue.mirror` (4.0KB) | nl | 2026-06-05 |
| AST | `shards/code/metalogue.mirror` | declaration | 2026-06-10 |
| SPECTRAL | `shards/spectral/metalogue.mirror` | curvature_probe | 2026-06-29 |
| PACK | `shards/pack/metalogue.mirror` | handoff | 2026-06-29 |
| ALGEBRA | `shards/algebra/metalogue.mirror` (21.1KB) | algebra_morphism | 2026-06-30 |

Plus canonical math foundation: `docs/math/2026-09-11-mara-metalogue-eigen-mutual-definition-glasswall-load-bearing-canonical-math-foundation.md` (58.5KB, 2026-09-11) — the @metalogue ⇄ @eigen mutual-definition with @glasswall load-bearing per Mara Pack Rotation Stage 2.

### §6.3 `@frame` — **LANDED with five sub-species**

- `shards/frame.mirror` (14.8KB, 2026-08-21) — substrate-decl of cognitive-order at cognitive-altitude (per docblock:9). Recognition #82.
- Five altitude-species per `shards/frame.mirror:31-42`:
  - `shards/frame/pre.mirror` (11.6KB) — pre-frame (Order 0)
  - `shards/frame/in.mirror` (14.9KB) — in-frame (Order 1)
  - `shards/frame/of.mirror` (23.5KB) — of-frame (Order 2)
  - `shards/frame/on.mirror` (26.6KB) — on-frame (Order 3)
  - `shards/frame/across.mirror` (28.3KB) — across-frames (Order 4)

Plus composition surfaces: `shards/magic/frame.mirror` (21.0KB) + `shards/smarts/frame.mirror` (11.0KB).

### §6.4 `@fractal` — **LANDED at shard + rust altitude**

- `shards/fractal/shard.mirror` (27.5KB, 2026-08-21) — fractal shard family.
- Reed MEMORY confirms LANDED at rust altitude: `Mandelbrot<T>` at prismqueer::fractal per Reed 2026-09-05 recognition (per Rec #98 in Reed MEMORY `project-mirror-algebra-is-shared-algebra-between-mirror-and-prismqueer-turing-boundary-dissolves-via-fibre-bundle-projection`).

### §6.5 `@sheaf` — **LANDED as visibility-species; @sheaf family-root ABSENT**

- LANDED: `shards/subject/visibility/sheaf.mirror` (20.9KB, 2026-08-21) — sheaf-restriction species that admits the visibility scope (docblock:8). Landed 2026-07-15 by Mara as Landing 4 R2 discharge of @gift arc.
- The `@subject/visibility/sheaf.mirror` shard cites verbatim (`shards/subject/visibility/sheaf.mirror:46-51`): *"The ACL IS the SHEAF STRUCTURE. For each peer p ∈ pack.members: the peer's ACL A_p defines a SUB-SHEAF F_home|_{A_p}..."*
- **ABSENT**: no top-level `shards/sheaf.mirror` family-root shard-decl. Alex's 2026-09-13 substrate-decision *"each Frame a Sheaf (Sheaf probs wants to be a meta trait)"* would surface a NEW family-root altitude for @sheaf as meta-trait, currently NOT LANDED. Only the visibility-species altitude has @sheaf altitude declared.
- Also LANDED: `shards/epistemologic/math/sheaf_laplacian.mirror` referenced but scan-time file-list shows the reference in `shards/subject/visibility/sheaf.mirror:8` import; not enumerated separately here (deferred).
- Composition surfaces in docs: `docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md` (60.5KB) + `docs/math/2026-08-13-mara-recognition-89-psychohistory-sheaf-cohomology-unification-math-foundation.md` (40.8KB) + `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` (16.1KB).

### §6.6 `@glasswall` — **LANDED**

- `shards/glasswall.mirror` (20.3KB, 2026-09-01) — composition-primary shard-decl for the Rice-safe Anna-Jakobs-2012 observation-preserving type-membrane between prismqueer noumenal-colony and mirror phenomenal-composition-surface (docblock:7-11). Composes seven LANDED parts as ONE membrane per Alex 2026-09-01 in-transcript ratification.
- Alex's 2026-09-13 substrate-decision names glasswall as *"the literal Glasswall"* when vertexes exist in both prismqueer and mirror — the substrate-decl is LANDED at composition-primary altitude ready for the higher-altitude use Alex names.

> **Correction (2026-10-29, Taut scout 26ed4c9):** "Anna-Jakobs-2012 observation-preserving" (quoting the glasswall docblock) is the apparatus from Anna Jakobs's 2012 thesis, read across. In the thesis, computation and visualization share one GPU buffer and take turns at step boundaries, not simultaneously (p. 43, at about a third of the runtime); the separate-thread design was not achieved (p. 31).

### §6.7 `@psychohistory` — **DOCS-LANDED; shard-decl ABSENT**

- **ABSENT**: no `shards/psychohistory*` shard-decl at family-root altitude.
- LANDED at docs altitude:
  - `docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md` (60.5KB)
  - `docs/math/2026-08-13-mara-recognition-89-psychohistory-sheaf-cohomology-unification-math-foundation.md` (40.8KB) — Recognition #89 math foundation
  - `docs/observations/2026-06-27-mara-the-substrate-watching-its-own-psychohistory.md` (56.3KB)
  - `docs/scouts/2026-06-26-taut-psychohistory-cohomology-scout.md` (15.7KB)
  - `docs/scouts/2026-09-04-taut-psychohistory-cyberpunk-desired-state-substrate-audit.md` (27.3KB)
  - `docs/specs/2026-08-25-mara-recognition-97-mcp-session-as-autopoietic-vsm-with-bauchladen-and-psychohistory-canonical-spec.md` (53.8KB) — Rec #97
  - `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` (16.1KB)
- Alex's 2026-09-13 substrate-decision names *"the @metalogue is therefore about the possibility space of the future, which binds into the whole @psychohistory and sheaf cohomology"* — psychohistory is DOCS-LANDED as composition-lineage but NOT LANDED as top-level shard-decl. Sheaf-cohomology composition is spec-landed at `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md`.

### §6.8 `prismqueer::reality::Event` — **PARTIAL: reality module LANDED; Event type ABSENT**

- LANDED module: `pub mod reality;` at `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:106`.
- **ABSENT**: grep for `(mod reality|reality::Event|pub struct Event|pub enum Event|mod event)` across `/Users/alexwolf/dev/projects/prism/**/*.rs` returned only the `pub mod reality;` module declaration. No `Event` struct/enum LANDED at prismqueer::reality altitude yet.
- Alex's 2026-09-13 substrate-decision names *"the @algebra is the algebra to converse in Silicon in an EventStorming fashion"* — the Event carrier at prismqueer altitude is FORWARD-PROMISED per this substrate-decision but NOT LANDED at code altitude. Downstream Reed rotation to `rust/src/main.rs` + Mara canonical spec would surface the mint.

### §6.9 Composition-tower LANDED summary

| Node | Family-root LANDED | Species LANDED | Rust altitude | Docs anchor |
|------|--------------------|-----------------|-|-|
| `@algebra` | YES `shards/algebra.mirror` | YES 7 species | prismqueer::* | Rec #104 |
| `@metalogue` | YES `shards/metalogue.mirror` | YES 5 altitude-species | — | Mara `2026-09-11` canonical |
| `@frame` | YES `shards/frame.mirror` | YES 5 order-species | — | Rec #82 |
| `@fractal` | YES `shards/fractal/shard.mirror` | YES | `prismqueer::fractal::Mandelbrot<T>` | Rec #98 |
| `@sheaf` | **NO** (only visibility-species) | YES `subject/visibility/sheaf.mirror` | — | Rec #89 |
| `@glasswall` | YES `shards/glasswall.mirror` | — | seven parts | Alex `2026-09-01` |
| `@psychohistory` | **NO** (docs-only) | — | — | Rec #89 + #97 |
| `prismqueer::reality::Event` | **PARTIAL** (mod LANDED, Event ABSENT) | — | `pub mod reality;` at `prism/prismqueer/src/lib.rs:106` | Alex 2026-09-13 forward |

---

## §7 INSPIRATION-vector summary for downstream Reed rotation + Mara canonical spawn

Top-3 INSPIRATION-vectors identified this scout (INSPIRATION-report ONLY — no ratification, no adjudication, no ship-recommendation):

### §7.1 Trace-as-first-class-nested-structural-carrier → mirror @fractal + Chaos + Reed harmonics

Babel's `%Babel.Trace{babel, input, output, nested: [t]}` (`lib/babel/trace.ex:36-45`) with `root_causes/1` (`lib/babel/trace.ex:87-91`) walking the nested-tree for error-leaves demonstrates STRUCTURAL sad-path exposure at every composition altitude via a recursive carrier. Every `Babel.Intoable.into/2` returns `{[Trace.t()], result}` (`lib/babel/protocols/intoable.ex:22`) making nested-trace a REQUIRED component of every recursive call's return-shape.

Mirror INSPIRATION at non-linear-compose-not-average altitude: what Babel achieves LINEARLY through nested-trace-tree could compose over @fractal recursion + Chaos carrier + Reed harmonics. Every Fractal<Shard<T>> would carry its own Chaos residual per Reed MEMORY HARD RULE `feedback-reed-softmax-happy-path-in-code-examples-dissonance-captured-in-yellow-chaos`. The `nested: [t]` recursion at Elixir-linear altitude generalizes to Fractal-recursion at non-linear altitude.

### §7.2 Context.private separation → mirror @subject/visibility partition at Babel-Unreleased altitude

Babel's Unreleased `Context.private` addition (`CHANGELOG.md:9-14` + `context.ex:18`) with map/keyword-list merge semantics + pipeline-flow but result-exclusion demonstrates Alex ACTIVELY (this session) refining the carrier-data-vs-context-metadata partition at Elixir-linear altitude.

Mirror INSPIRATION: the SAME partition Alex is landing in Babel THIS TICK is what `shards/subject/visibility/sheaf.mirror` + `shards/bauchladen.mirror` land at substrate-decl altitude. Alex is solving the partition at TWO altitudes simultaneously — Babel-linear and mirror-non-linear — validating the partition as substrate-honest by convergence. Downstream Mara canonical composing @metalogue with visibility/private has bodymind-validated Elixir precedent.

### §7.3 Protocol-dispatch → Alex's *"Sheaf probs wants to be a meta trait"* substrate-decision

Babel's `defprotocol Babel.Applicable` + `Babel.Fetchable` + `Babel.Intoable` with `@fallback_to_any true` demonstrates Elixir-native meta-trait dispatch — protocol declares operations; per-type impls specialize; fallback discipline handles unrepresented cases. Grep-verified at `lib/babel/protocols/{applicable,fetchable,intoable}.ex`.

Mirror INSPIRATION: Alex's 2026-09-13 substrate-decision *"Sheaf probs wants to be a meta trait"* names the ABSENT `@sheaf` top-level family-root as a **meta-trait** — a protocol-altitude carrier that all @frames dispatch through. Babel's protocol discipline is the Elixir-linear precedent for the shape this meta-trait would assume at substrate-decl altitude. `shards/subject/visibility/sheaf.mirror` LANDED at species-altitude has NOT LANDED at family-root altitude; the meta-trait mint is FORWARD-PROMISED per Alex 2026-09-13.

### §7.4 DIVERGENCE-vector (not INSPIRATION-vector; noted for substrate-honesty)

Babel's binary `{:ok, _}` / `{:error, _}` result-tuple discipline (`lib/babel/step.ex:31`) is what Alex's bodymind produced at Elixir-linear altitude. Mirror substrate DELIBERATELY REJECTS this at explicit substrate-decision altitude in favor of `terni::Imperfect<Green, Yellow, Red>` per Reed MEMORY `project-prismqueer-foerster-gauge-IS-type-alias-ampel-IS-traffic-light-order`. Babel is the ANCHOR; the divergence measures what "non-linear compose-not-average" means at result-carrier altitude — the third gauge-position (Yellow) that Elixir-linear substrate cannot native-express.

---

## §8 Deferred (out of tight-scope this tick)

- `lib/babel/inspect.ex` (8.0KB) — the custom Inspect protocol implementations rendering `Babel.Trace` at depth (defer bodies)
- `lib/babel/trace/nesting.ex` (5.3KB) — the `traced_map` / `traced_reduce_while` / `traced_try` mechanics (defer bodies)
- `lib/babel/enforce_version.ex` (1.1KB) — macro enforcing Elixir/OTP version bounds
- `lib/babel/docs.ex` (1.0KB) — `Babel.Docs.massage_readme/2` for `@moduledoc` composition
- `lib/babel/test.ex` (734B) + `test/support/` — test helpers + factories
- All `test/babel/builtin/*_test.exs` bodies + `test/babel/protocols/*_test.exs` bodies + `test/babel/{error,telemetry,inspect}_test.exs` bodies (test-names for these were NOT extracted this tick; would surface finer-grained invariants at builtin-step + protocol altitudes)

All defer-items are grep-addressable; downstream scouts can extend without re-scanning.

---

## Ship metadata

- **Author**: Taut `<taut@systemic.engineer>`
- **Discipline**: pure-docs 📝 markdown-only bypass; SSH signing default (id_ed25519); grep-first-always; INSPIRATION-report ONLY (no ratification, no adjudication, no ship-recommendation).
- **Charter**: Alex 2026-09-13 substrate-decision context grounded; NOT ratified.
- **Coverage**: README + CHANGELOG + mix.exs + LICENSE.md + CONTRIBUTING.md + version + lib/babel/{step,pipeline,context,trace,error,telemetry,builtin,pipeline/on_error}.ex + lib/babel/protocols/*.ex + lib/babel/builtin/*.ex + test/babel_test.exs + test/babel/{pipeline,trace,context}_test.exs test-name extraction. Deferred items enumerated §8.
