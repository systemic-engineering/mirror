# Benchmark Tracing — Tracked Loss as the Performance Wire

*2026-06-17. Taut. Spec — pinning the surface that turns `mirror`'s
already-substrate-declared loss algebra into a richer tracing signal
than `erlang:trace/3` provides, and naming the bench harness that
collects it.*

Status: **Red.** Surface designed; one composition tick from
`@lens/transit` + `@mirror/loss/transparency` + `@kintsugi/oscillate`
landing the wire. Implementation = a `mirror bench <target>` invocation
that emits substrate-text trace lines and an aggregate
`transparency<benchmark>` verdict.

Depends on:

- `shards/epistemologic/pact/benchmark.mirror` — the seven verdict
  properties (no_hang, linear_compile, binary_size, deterministic_oid,
  cache_speedup, incremental_speedup, l1_resident). Lifted from
  `boot/std/epistemologic/property/benchmark.mirror` this session
  (recognition #37 closure: `grammar` → `pact`, path-root match per
  `@epistemologic/pact/keyword_matches_path_root`).
- `shards/mirror/loss.mirror` + `shards/mirror/loss/transparency.mirror`
  (Mara, 2026-06-12) — the loss family root + canonical carrier.
  `transparency<p>` IS the trace event's payload type. `dark_dims(loss)`
  IS the structural projection (which axes failed, not just how much).
- `shards/mirror/spectral/score.mirror` (Mara, T11.1, 2026-06-10) —
  the orchestra-altitude shared score the pulse-by-pulse trace
  events project onto. `score = (anchor, session, pending)`; benchmark
  trace IS the temporal projection of the same record.
- `shards/kintsugi/oscillate.mirror` (Mara, T11, 2026-06-10) — the
  five-state oscillation_state surface. Each ACTIVE/DARK pulse is
  one trace event with the located transparency at that altitude.
- `fragmentation/docs/specs/lens-transit.md` (Taut, 2026-06-01) —
  the six-axis hardware-floor measurement glass. `@lens/transit` IS
  the @io-floor instrument; this spec sits one altitude above,
  composing `transit` reports into the substrate's loss algebra.
- `docs/insights/2026-06-06-benchmarking-glass-sweep.md` (Taut,
  2026-06-06) — the recognition that `@lens/transit` + benchmark
  properties + `@mirror/refract` + `@epistemologic/silicon/compute_bound`
  together constitute the benchmarking glass. This spec wires the
  four into one harness.
- `docs/benchmarks/baseline-rust.md` (2026-05-17) — the ground-truth
  stopwatch numbers this surface replaces with structured
  transparency<benchmark> verdicts. Numbers stale beyond cardinality
  (the 717KB binary; the ~1.42s `craft boot` wall time). Re-measure
  when the harness lands; until then, treat that file as historical
  baseline.

Substrate decisions cited:

- [[architecture-prism-as-trait-as-everything]] — the benchmark
  harness IS a prism; `bench` is one species under `@mirror/lens`
  (sibling to cli / shell / mcp / lsp / transit). The five-operation
  block (focus / project / split / shift / settle) IS the canonical
  declaration form.
- [[architecture-connes-spectral-triple]] — the harness sits at the
  spectral-triple's (A, H, D) reading: A = the five operations under
  measurement; H = the substrate's running document state; D = the
  Dirac the loss algebra picks (transparency at this altitude;
  forward-promised shannon/dirichlet/massey at sibling altitudes).
- [[architecture-fragmentation-is-the-rust-substrate]] — the @io
  floor (the actual subprocess / syscall layer that emits trace
  bytes) lives in the Rust crate (`bootstrap/src/bench.rs`,
  forward-promised); the substrate names the dispatch contract.
- [[architecture-prediction-paradigm-orthogonal-to-optimization]] —
  the harness is on the orthogonal axis to throughput-tuning. It
  observes the substrate's *gap algebra* under run; a benchmark
  verdict says WHERE structural-loss accumulates, not just how fast
  bytes flow.

---

## 0. Headline

**BEAM's `erlang:trace/3` records WHERE time goes. Mirror's tracked
loss records WHERE coherence-loss goes.**

The two surfaces are sibling instruments at different altitudes.
BEAM's trace is a time-stamped event stream on processes; mirror's
trace IS a transparency-stamped event stream on prisms. Both compose
ACTIVE/DARK alternation (BEAM's running/waiting; oscillate's
active/dark/settled/escalated/paused-by-half); both compose
per-operation. The difference is the *payload*: BEAM carries
microseconds; mirror carries `transparency<p>` with `opacity_map`
located at the substrate-altitude failure site.

The benchmark surface this spec pins:

1. Records every prism operation's `transparency<benchmark>` verdict.
2. Aggregates per-axis `opacity_map` over a kintsugi run.
3. Composes the per-pulse losses into a corpus-level verdict via the
   substrate's existing Transparency monoid (Fail-dominates /
   Partial-min-confidence / Pass-neutral; no new combine).
4. Emits substrate-text (`.mirror` format) as the wire — JSON lives
   at @io only, per [[feedback-no-stringly-types]] and Mara's
   `--out @data/json` pattern (commits `b871a00` / `34ca0d5`).
5. Plays back into `@kintsugi/oscillate.pulse` as one (active, dark)
   pair per benchmark iteration — the loop closes; the verdict is
   the input to the next iteration's altitude selection.

Nothing in this spec is new vocabulary. The substrate already had
the words (the 53rd-ish instance of
[[feedback-substrate-already-had-the-word]] — `bench` is one of the
optical schematic keywords per the depth>=1 inventory in
`@epistemologic/pact/keyword_matches_depth`; `transit` is the
measurement carrier; `oscillate` is the loop). This spec names how
they compose.

---

## 1. The trace event

### 1.1 Shape

One trace event per prism operation invocation. The event carries:

```mirror
type trace_event = {
  # Monotonically increasing per-run. Lifts into the substrate's
  # @time.duration carrier when @time grounds; today a u64 step
  # counter is the floor.
  iteration: u64,

  # The prism operation this event measures. One of the five:
  # focus | project | split | shift | settle. The substrate's
  # five-operation algebra IS the event taxonomy; no extra event
  # types beyond what the algebra names.
  operation: prism_operation,

  # The substrate ref of the call site (the shard + the keyword
  # being invoked). Located transparency means located events;
  # this field is the lens-projection of the call site's
  # substrate position.
  origin: ref,

  # The full transparency<benchmark> verdict for this operation.
  # success | partial(opacity_map) | failure(opacity_map). Each
  # opacity in the map names ONE offending sub-altitude; the
  # composition law is Transparency<P> monoid (Fail-dominates).
  verdict: transparency,

  # The six axes lens-transit measures, lifted as a structured
  # sub-record. WallClock / FpPrecision / CachePressure /
  # Allocation / BranchMisses / BudgetConsumption. Per
  # fragmentation/docs/specs/lens-transit.md §1.1. Each axis IS
  # a property under transparency<benchmark>; the joint report
  # IS the verdict above.
  transit: lens_transit_report,

  # The oscillation state at this event, per
  # shards/kintsugi/oscillate.mirror's five-state surface.
  # `active` and `dark` carry distinct semantics: ACTIVE pulse
  # = loss-decreasing proposal under measurement; DARK pulse
  # = identity anchoring (the substrate verifies the post-event
  # OID byte-equals the pre-event OID's DARK 80 bits). The
  # benchmark consumer reads both columns to distinguish work
  # done from work that left identity invariant.
  pulse_state: oscillation_state,

  # The dark_count delta. Lifts `bootstrap/src/property.rs`'s
  # AstKind::Dark counter to the trace altitude. Each Dark
  # node introduced or settled in this event contributes ±1;
  # the running sum across events IS the corpus-altitude
  # transparency monoid's residual weight.
  dark_delta: i64,
}
```

`trace_event` is declared once at `shards/mirror/bench/event.mirror`
(forward-promised; this spec's first implementation tick). The
record discipline follows the existing `score` precedent in
`shards/mirror/spectral/score.mirror` (three named fields, each a
typed carrier from a sibling shard) — Mara's T11.1 shape, lifted
to the benchmark altitude.

### 1.2 What the event carries that `erlang:trace/3` doesn't

`erlang:trace(Pid, true, [call, return_to, send, 'receive'])` emits
a stream of `{trace, Pid, call, {M,F,A}}` / `{trace, Pid, return_to,
{M,F,A}}` / `{trace, Pid, send, Msg, To}` records. The payload is
ordering + a timestamp + minimal data. The forensic value is the
sequence; the structural-loss value is zero — BEAM trace cannot tell
you WHERE coherence broke, only WHERE time went.

Mirror's trace_event carries:

- `verdict: transparency<benchmark>` — the structural-loss verdict
  for the operation. Names WHICH of the seven properties (no_hang,
  linear_compile, binary_size, deterministic_oid, cache_speedup,
  incremental_speedup, l1_resident) failed; the opacity_map locates
  the failure to a sub-altitude (e.g. `linear_compile` failed at
  `tokenize` not `content_oid`).
- `transit: lens_transit_report` — the six hardware-floor measurements
  with the floor annotations attached. `WallClock=145ns | floor=1ns`
  is one column; `CachePressure=3 lines evicted | floor=64B/line` is
  another. The reader knows what was unobservable.
- `pulse_state: oscillation_state` — the loop position. Distinguishes
  ACTIVE work (the substrate proposing a loss-decreasing morphism)
  from DARK anchoring (the substrate re-validating identity invariance).
  BEAM has running/waiting; oscillate's five states are stricter.
- `dark_delta: i64` — the structural debt delta. AST nodes the
  operation either created as Dark or settled to a typed AST kind.
  Running sum IS Banach distance to ker(Δ₀).

The BEAM-side reading: BEAM's events are *temporal sequences over
processes*; mirror's events are *transparency verdicts over
prisms*. Trace one of them in isolation and you have stopwatch data;
trace both columns and you have a structural diagnosis.

---

## 2. The collector

### 2.1 Default: instrumented runs only

The harness does NOT trace every run. Default `mirror` runs (build,
craft, kintsugi, kintsugi --ci, kintsugi mirror.spec) emit no
trace_event records. The substrate's existing kintsugi verdict
envelope (per `boot/std/kintsugi.mirror`'s `type verdict = { ... }`
declaration; T11.2.6 substrate-pull closure) is what runs land by
default.

Trace events are produced when:

1. `mirror bench <target>` is invoked explicitly. The `bench`
   subcommand IS the new CLI surface this spec adds.
2. `mirror kintsugi --bench <target>` (forward-promised flag) lifts
   any kintsugi run into instrumented mode. Same dispatcher; one
   extra column emitted.

### 2.2 Why not always-on

Two reasons grounded in the substrate:

1. **Cost is non-zero.** Per [[architecture-fragmentation-is-the-rust-substrate]]
   the @io floor pays for syscalls and `clock_gettime(CLOCK_MONOTONIC)`
   reads. Always-on tracing inflates wall-clock and erases the
   measurement (a Heisenberg cost). `@lens/transit`'s hardware-floor
   discipline (§1.3 of `lens-transit.md`) says: never measure below
   the floor; never inflate above it. Optional tracing keeps the
   floor honest.
2. **Trace is a lens, not a load-bearing property.** Per
   [[architecture-shards-as-substrate-source]] the substrate's
   running state IS the truth; trace is a projection through a typed
   surface. The five operations don't change shape when traced;
   trace only adds the observation lens. Mandatory tracing would
   conflate the lens with the substrate.

### 2.3 What `mirror bench <target>` does

The subcommand:

1. Locates `<target>` (a `.mirror` file, a directory, or
   `mirror.spec`). The dispatch follows the existing kintsugi-corpus
   walker (`bootstrap/src/lib.rs:cmd_kintsugi_ci_corpus`); the bench
   walker is the same code path with `--bench` toggled.
2. Loads the seven benchmark properties from
   `@epistemologic/pact/benchmark` (the migrated shard) per
   Mara's cross-shard resolver (commit `128e0d2`).
3. Resolves the `transit` measurement glass through
   `@lens/transit`'s six-axis property family. Where the floor is
   below hardware precision, the column reports `unobservable` not
   `0` — per `lens-transit.md` §1.3.
4. Runs the target once per benchmark iteration. Each operation
   along the way emits one `trace_event`. The events stream to
   stdout (mirror-text) or to a substrate-ref the user names
   (`--out @data/trace` per Mara's `--out` discipline,
   commits `b871a00`/`34ca0d5`).
5. Aggregates the events into one corpus-level
   `transparency<benchmark>` verdict via the existing Transparency
   monoid. Same aggregation rules as kintsugi-ci's per-file →
   corpus envelope (T11.2.5/T11.2.6).
6. Exits 0 when `verdict != failure`; non-zero with stderr
   diagnostics when `failure(opacity_map)` is non-empty. The exit
   semantics borrow from `just kintsugi-ci-local` (Justfile lines
   207-219).

---

## 3. The wire format

Per [[feedback-no-stringly-types]] and Mara's recent `--out @data/json`
work (substrate-ref dispatch lands at `b871a00`): the wire is
**substrate-text by default**, JSON only at the @io boundary when a
consumer explicitly pulls `--out @data/json`.

### 3.1 Substrate-text shape

One trace_event per `event { ... }` block in declaration order:

```mirror
event {
  iteration   1
  operation   focus
  origin      @mirror/spec/source
  verdict     success
  transit     {
    wall_clock_ns   145
    fp_precision    machine_epsilon
    cache_pressure  0
    allocation      0
    branch_misses   2
    budget_consumed 1
  }
  pulse_state active
  dark_delta  0
}

event {
  iteration   2
  operation   project
  origin      @mirror/spec/target
  verdict     partial({
    "linear_compile" => "tokenize: 9120 tokens at 12.3µs (O(n) bound: 10.0µs)",
  })
  transit     {
    wall_clock_ns   12300
    fp_precision    machine_epsilon
    cache_pressure  3
    allocation      4096
    branch_misses   18
    budget_consumed 4
  }
  pulse_state active
  dark_delta  0
}

# ... events 3..N ...

corpus_verdict {
  total_iterations   N
  total_dark_delta   0
  verdict            partial({
    "linear_compile" => "1/N events over O(n) bound",
  })
  per_property {
    no_hang             success
    linear_compile      partial(...)
    binary_size         success
    deterministic_oid   success
    cache_speedup       success
    incremental_speedup success
    l1_resident         success
  }
}
```

This is the same record-emission shape T11.2.5/T11.2.6 lifted for
the kintsugi-ci envelope; this spec keeps the precedent (each
`event { ... }` is a record per the substrate's existing
verdict-record discipline).

### 3.2 JSON shape (at @io only)

When invoked with `--out @data/json`, the dispatcher routes the
same record sequence through Mara's substrate-ref → json projection
(`bootstrap/src/lib.rs:emit_corpus_verdict_json`, the JSON path
preserved alongside the mirror-text default). The JSON keys follow
the same field names as the substrate-text shape (Mara's discipline:
JSON `path` ↔ mirror-text `file` etc. is the @io-boundary's
responsibility; the substrate declares the field names).

### 3.3 Why substrate-text by default

Per [[feedback-no-stringly-types]]: JSON keys carry no substrate
type — a trace consumer reading JSON has to round-trip back to
the substrate to know what `linear_compile` means. Substrate-text
keeps the trace IN the substrate's type system; consumers read
the same shape they'd write at the `pact` declaration altitude.
The kintsugi-ci envelope precedent (T11.2.5: mirror-text is
default; JSON is @io-only) lands here verbatim.

---

## 4. BEAM analogy table — what richer signal mirror gives

| Column                       | `erlang:trace/3`                                          | Mirror benchmark trace                                                                                  |
| ---------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Unit of observation          | OS process (Pid)                                          | Prism operation (focus/project/split/shift/settle)                                                      |
| Event taxonomy               | call / return_to / send / receive / garbage_collection    | Five operations × five pulse states (active/dark/settled/escalated/paused-by-half)                      |
| Payload                      | Timestamp + minimal data (MFA, message, GC stats)         | transparency<benchmark> verdict + lens_transit six-axis + pulse_state + dark_delta                      |
| Composition law              | Append-only event log; reader reassembles                 | Substrate Transparency<P> monoid (Fail-dominates / Partial-min-confidence / Pass-neutral)               |
| Locality of failure          | Inferred from MFA + stack at trace time                   | Substrate-altitude `opacity_map` names the failing sub-altitude (e.g. `tokenize` under `linear_compile`)|
| Hardware-floor honesty       | None — wall-clock is the only floor                       | Per `lens-transit.md` §1.3: each of six axes annotates its hardware floor at the measurement site       |
| Structural-debt accounting   | None                                                      | `dark_delta` per event + cumulative residual = Banach distance to ker(Δ₀) (eⁿ⁺¹ < eⁿ)                  |
| Identity preservation        | None                                                      | DARK pulse re-validates the post-event OID's DARK 80 bits byte-equal the pre-event's (per oscillate.mirror)|
| Loop closure                 | External consumer reads the trace and decides next steps  | The trace feeds back into `@kintsugi/oscillate.pulse` (the loop reads its own verdict)                  |
| Replayability                | Time-ordered; replay requires re-creating process state   | Content-addressed via @mirror/store; replay reads the OID graph (per [[architecture-three-tier-stack]]) |
| Aggregation                  | Manual fold over `{trace, Pid, ...}` records              | Substrate Transparency monoid composes events into per-property verdict; per-property into corpus       |

The BEAM precedent that DOES translate cleanly: per-process tracing
is per-prism tracing; selective enabling is selective enabling
(`erlang:trace_pattern(MFA, [{...}])` ↔ `mirror bench --only @<ref>`).
The precedent that DOES NOT: BEAM's trace is a debugging instrument
that adds visibility AFTER the system has a problem. Mirror's trace
is a structural-loss observation that is the SUBSTRATE'S own algebra
projected through a lens — same kind of signal that runs the kintsugi
loop, named at the benchmark altitude.

---

## 5. Numerical targets — what the benchmark chases

The seven properties from `shards/epistemologic/pact/benchmark.mirror`
have inline thresholds. The harness reads them as the GAME; each
benchmark iteration's verdict is the score against those thresholds.

| Property               | Threshold                                            | Current baseline (2026-05-17)            |
| ---------------------- | ---------------------------------------------------- | ---------------------------------------- |
| `no_hang`              | exec completes within wall-clock bound; no unbounded stdin | passes (no hang observed)            |
| `linear_compile`       | parse + content_oid O(n) in tokens / nodes            | per-file 1.3-75ms; not yet bounded       |
| `binary_size`          | stripped binary < 64KB                                | 591KB stripped (12× over threshold)      |
| `deterministic_oid`    | identical AST → identical OID; no FP-contraction drift | passes (`-ffp-contract=off` set)        |
| `cache_speedup`        | git-store hit > 2× faster than cold compile           | 80/81 cache hits at 1.42s wall; ratio TBD |
| `incremental_speedup`  | incremental craft < full craft × (changed / total)    | not yet measured                         |
| `l1_resident`          | text section < 32KB; hot path < 16KB                  | not measured; binary is 12× over L1      |

Plus one structural property the harness adds — the **bounded
transparency loss per iteration** — per
[[architecture-kintsugi-bias-lift]]'s Banach contraction reading:

```
forall iteration i:
  weight(transparency_at(i+1)) <= ρ · weight(transparency_at(i))
  where 0 < ρ < 1   (Polyak-Łojasiewicz contraction rate; per
                     `@mirror/loss/dirichlet`'s sheaf-energy descent
                     reading when that sub-glass lands)
```

This is the harness's load-bearing claim: not just "go faster" but
"every iteration's structural-debt weight decreases by a bounded
factor." When iteration N+1 weighs *more* than iteration N, the
benchmark verdict transitions to `failure(opacity_map)` and the
opacity names which property regressed.

---

## 6. Integration with `@kintsugi/oscillate`

The kintsugi loop IS the benchmark target. Per
`shards/kintsugi/oscillate.mirror`, each `pulse()` invocation is
one (ACTIVE pass, DARK pass, consent-read) triple. Per the loss
recognition above, each pass produces one trace_event. So:

```
   pulse N        : ACTIVE → event_{2N-1}     verdict_a
                     DARK  → event_{2N}       verdict_d
   consent read   : reads consent.query_phi   (no event; gate-only)
   pulse N+1      : ACTIVE → event_{2N+1}     verdict_a'
                     DARK  → event_{2N+2}     verdict_d'
   ...
   settled        : oscillation_state = settled
                     corpus_verdict emitted; harness exits
```

This means:

- The benchmark IS the kintsugi loop, instrumented. The harness
  doesn't add a parallel loop; it adds the trace_event emission
  to the existing pulse boundary.
- The Transparency monoid composing the per-pulse verdicts IS the
  same monoid composing the kintsugi loop's overall settlement
  read (per `shards/kintsugi/consent.mirror`'s gate+rank surface).
  One algebra; two readings.
- Termination is the existing five-state oscillation surface;
  `settled` → emit corpus_verdict, exit. `escalated` / `paused-by-half`
  → emit a `partial` corpus_verdict with the opacity_map naming the
  pause reason; exit non-zero per the pre-commit gate's discipline.
- The harness adds NO new termination logic. The loop's cadence is
  the loop's; the benchmark observes.

Per [[architecture-kintsugi-loop-altitude-portable]] (recognition
#59, promoted 2026-06-11): the kintsugi loop is altitude-portable.
The benchmark surface IS one altitude (the build/run altitude); the
same loop runs at the per-keyword altitude (the kintsugi/fracture/
shards) and at the per-shard altitude (kintsugi-ci). The trace_event
shape is the same at every altitude; only the property family
parameterizing `transparency<p>` differs.

---

## 7. Comparison to the seven benchmark properties

Each property in `shards/epistemologic/pact/benchmark.mirror` is
declared with surface `name(args) -> transparency { \ }`. Today
the obligation block is unresolved; the harness lifts each property
into a verdict OVER the trace. The lift discipline:

| Property               | Trace-altitude lift                                                                                                          |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `no_hang(ast)`         | success iff every `trace_event.transit.wall_clock_ns < wall_bound_ns`; failure(opacity_map) names the offending operation+origin |
| `linear_compile(ast)`  | success iff regression slope of `wall_clock_ns ~ token_count` has R² > 0.95 with slope > 0; failure if quadratic-fit beats linear |
| `binary_size(b, t)`    | success iff `du -b $binary < t`; not measured per-event (one-shot post-build read); recorded as the harness's pre-run preamble    |
| `deterministic_oid`    | success iff `content_oid(ast) == content_oid(ast)` across N invocations; failure(opacity_map) names the diverging OID pair       |
| `cache_speedup(file)`  | success iff `mean(cold_runs.wall_clock) / mean(warm_runs.wall_clock) > 2.0`; partial if 1.0 < ratio < 2.0                         |
| `incremental_speedup`  | success iff `wall(incremental) < wall(full) * (changed_files / total_files)`; failure if the inequality reverses                 |
| `l1_resident(b)`       | success iff `text_section_bytes(b) < 32 * 1024`; partial if `< 64 * 1024`; failure otherwise                                     |

Each lift IS the body of the property's `\` obligation block; today
the lifts are spec-described and the harness performs them in Rust
glue (per [[architecture-fragmentation-is-the-rust-substrate]]:
the @io floor lives in the crate). Forward-promised fracture
bodies under `@kintsugi/fracture/benchmark` lift each into substrate
declaration via `splinter(ast)` (per
[[architecture-splinter-ast-quote-primitive]]); not this tick.

---

## 8. Minimal validation — one concrete number

The smallest end-to-end invocation that proves the spec is
operationalizable:

```bash
# Build the bench-capable binary (a stub harness; one property
# wired; the rest forward-promised). Lives in `bootstrap/src/bench.rs`.
cargo build --release --manifest-path bootstrap/Cargo.toml

# Run the binary_size property against the just-built binary.
# Substrate-text out by default; one concrete number emitted as
# transit.binary_bytes.
mirror bench binary_size --binary /Users/reed/.cargo-target/release/mirror

# Expected verdict (against today's baseline):
# event {
#   iteration   1
#   operation   focus
#   origin      @epistemologic/pact/benchmark.binary_size
#   verdict     failure({
#     "binary_size" => "591KB > 64KB threshold",
#   })
#   transit     {
#     binary_bytes 591872
#     threshold    65536
#   }
#   pulse_state active
#   dark_delta  0
# }
# corpus_verdict { verdict failure ... }
# exit 1
```

This proves the wire: one property, one trace_event, one corpus
verdict, one exit code. The other six properties extend the same
shape (the harness picks them up as `bench *` once the per-property
glue lands).

The minimum implementation tick to land this:

1. `shards/mirror/bench.mirror` — the family root declaring
   `prism @mirror/bench` with the five operations and the
   `bench(target: ref) -> transparency<benchmark>` action.
2. `shards/mirror/bench/event.mirror` — the `trace_event` record
   declaration per §1.1.
3. `bootstrap/src/bench.rs` — the Rust @io-floor dispatcher;
   `cmd_bench()` walks the target, invokes per-property lifts,
   emits trace_events. ~150 LOC; same shape as
   `cmd_kintsugi_ci_corpus`.
4. CLI wiring in `bootstrap/src/lib.rs:cmd_dispatch` — one
   subcommand match arm. ~10 LOC.

Total tick: ~400 LOC across substrate + Rust @io floor + one
test. Per the kintsugi-ci-v0.1 precedent (T11.2.5 envelope, T11.3
corpus walker), this is one well-scoped iteration of the harness.

---

## 9. What this spec does NOT cover

- **The transit @io floor itself.** `@lens/transit`'s `clock_gettime`
  / `perf_event_open` / `mach_absolute_time` wiring lives in
  `fragmentation`. This spec assumes that floor lands separately;
  until it does, the transit columns degrade gracefully to
  `unobservable` (per the hardware-floor discipline).
- **Cross-hardware verdict comparison.** Per
  `lens-transit.md` §1.3 — deferred. The harness records the
  hardware floors that applied; cross-machine comparison is a
  future tick.
- **Continuous tracing in production.** Per §2.2 above, always-on
  tracing inflates measurements. Production runs land their
  verdicts without trace; the verdict envelope already carries the
  weight info. The benchmark harness is an explicitly-invoked
  diagnostic.
- **Migration of the remaining 17 `boot/std/epistemologic/property/*`
  shards.** This spec migrated `benchmark.mirror` only (the one it
  consumes); the broader migration of `halts`, `autopoietic`,
  `content_addressed`, `glass_wall`, `frame_relativity`,
  `coincidence_matches`, `duplicate_variant`, `filename_matches_glass`,
  `io_safety`, `is_prism_record`, `total_classification`, and the
  six `laws/*` shards is forward-promised. Those files use the
  retired `grammar` keyword and the deprecated `@epistemologic/property/`
  path; they're dark under `@epistemologic/pact/keyword_matches_path_root`
  per its own §"For the dark shards still under
  `@epistemologic/property/<X>`" framing. A future Mara/Reed tick
  takes the cascade across both the substrate paths and the five
  Rust string constants (`bootstrap/src/{property,score,kintsugi,
  tensor,oscillate}.rs`) that hardcode
  `@epistemologic/property/total_classification`.
- **The `@kintsugi/fracture/benchmark` body.** Forward-promised per
  §7 above. Each of the seven properties' lifts is declared in
  this spec; the substrate-altitude fracture body that discharges
  each `\` via `splinter(ast)` is a sibling tick.

---

## 10. Forward-promises

| Item                                         | Owner    | Substrate ref                                                |
| -------------------------------------------- | -------- | ------------------------------------------------------------ |
| `shards/mirror/bench.mirror`                 | Taut     | `@mirror/bench`                                              |
| `shards/mirror/bench/event.mirror`           | Taut     | `@mirror/bench/event`                                        |
| `bootstrap/src/bench.rs` (@io floor)         | Taut     | n/a (@io)                                                    |
| `@kintsugi/fracture/benchmark` (×7)          | Mara     | `@kintsugi/fracture/benchmark/<property>`                    |
| @lens/transit @io clock wiring               | Reed     | `fragmentation/vcs/mcp/transit/` (cross-repo)                |
| Migration of remaining property/* shards     | Mara/Reed| `@epistemologic/pact/{halts,autopoietic,…,laws/*}` (×17)     |
| Refresh of `docs/benchmarks/baseline-rust.md`| Taut     | n/a (operator-facing markdown)                               |

---

## 11. The proof

`eⁿ⁺¹ < eⁿ`.

Per the CLAUDE.md framing: the system learns from its errors; the
errors get smaller; the growth is monotonically non-decreasing by
convexity. The benchmark harness IS that proof made measurable.
Each pulse produces a transparency verdict; the verdict's weight
is the residual error; the next pulse's weight is bounded below
the current's by ρ < 1. The harness names the inequality,
records the witness, exits non-zero when the witness fails.

BEAM's `erlang:trace/3` cannot say this. It records WHERE time
goes, not WHETHER the system is converging. The substrate's
tracked loss IS the convergence measurement. The benchmark harness
is the lens that projects the substrate's running convergence into
a wire format an operator (or the next iteration of the kintsugi
loop) can read.

The benchmark IS the game. The game IS the proof.
