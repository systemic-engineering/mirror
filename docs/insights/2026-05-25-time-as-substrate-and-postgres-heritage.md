# Time as substrate: @time.duration and the PostgreSQL heritage

*2026-05-25. Reed (research).*

Status: **Yellow** — recognition complete; @time.duration shape proposed; @epistemologic property carriers named. No grammar lands in this tick; design substrate for the next.

Depends on:
- `mirror/boot/std/time.mirror` — the existing `@time` grammar (tick + snapshot + timeline + cursor + delta + replay + fork).
- `mirror/boot/std/beam.mirror` — declares `type duration(u64)` as a BEAM-local carrier; `beam` records carry `emitted: tick, duration: duration`.
- `mirror/boot/std/epistemologic/silicon/compute_bound.mirror` — Mara's α/5 (#65) parked `type wall_time = u64` locally rather than import @beam.duration; the carrier the design call deferred.
- `mirror/boot/std/epistemologic/property.mirror` — the verdict algebra all property checks return.
- `mirror/docs/specs/shard-design.md` §3 — `@mirror/shard/self` as observer-relative λ₀; comparing across shards is cross-observer.
- `mirror/docs/insights/2026-05-25-shard-as-observer-relative-lambda-zero.md` — observer-relativity at the substrate level.
- `mirror/docs/insights/2026-05-25-parametric-types-and-fp-heritage.md` — the `zoom(T)` / `refract(T)` discipline `duration` must honour.

---

## 1. Thesis

**Time is substrate, not utility.** PostgreSQL has spent twenty-eight years (since 1997's 6.0) learning that date/time is the place where naive type systems break first and break worst: timezone confusion, DST cliffs, leap seconds, monotonic-vs-wall drift, interval arithmetic that silently corrupts under daylight transitions. Their hard-won design — `timestamptz` vs `timestamp`, `interval` with named fields, range types with inclusivity bounds, the deliberate refusal to add leap seconds to `epoch` — is the closest thing in working software to a finished spec for how time should be typed. Mirror's algebra has the structural pieces to do it one altitude higher: `@epistemologic/property/*` carries the laws as verdicts; `@mirror/shard/self`'s observer-relativity carries timezone-as-frame; `zoom(T)` carries the annotation-only refinement (`duration<wall>` vs `duration<monotonic>` is phantom, not runtime); `refract(T)` carries verified construction (a `duration` whose existence witnesses non-negativity). The shared `@time.duration` Mara parked at α/5 is not a u64-with-units; it is the **algebraic home** for the substrate-level time concerns the rest of the system has been pretending are someone else's problem.

---

## 2. What PostgreSQL has figured out

These are the concrete concerns PG handles correctly. Each is a lesson mirror's algebra inherits or refutes:

**2.1 timestamptz vs timestamp (PG docs §8.5).** `timestamp without time zone` is a wall-clock face value with no frame; `timestamp with time zone` is a UTC instant *stored* as a fixed offset and *rendered* in the session's `TIMEZONE`. PG learned the hard way: storing local time is almost always wrong because the frame is ambient and changes. The rule is unambiguous: use `timestamptz` unless you're modelling a recurring local event (a calendar alarm at 09:00 every weekday). Mirror's analogue: timestamps **must** carry their observer frame; the shard's `self` provides it.

**2.2 interval (PG docs §8.5.1.4).** PG's `interval` is *not* a duration. It's a triple `(months, days, microseconds)` because **months and days have no fixed length in seconds** (DST + variable month length). Interval arithmetic is *frame-dependent*: `'2024-03-09 02:00 America/Los_Angeles' + interval '1 day'` is not `'2024-03-10 02:00 PST'` because of the DST spring-forward — PG resolves it as `'2024-03-10 03:00'`, the same wall-clock face value. This is the deepest PG lesson: **duration arithmetic is not associative across frames**.

**2.3 range types (PG docs §8.17).** `tstzrange(lower, upper, '[)')` carries inclusivity per endpoint, supports `&&` (overlap), `@>` (contains), and the `multirange` extension composes disjoint ranges. Range types are *the* substrate for budgets, validity windows, scheduled jobs, and event-sourced state. Mirror's spec mentioned `option(duration)` for budgets; that's the degenerate case of a `tstzrange` with `[0, max]`.

**2.4 monotonic vs wall (PG ≥ 14 internals).** PG uses `clock_gettime(CLOCK_MONOTONIC)` for query timing, `CLOCK_REALTIME` for `now()`. They never confuse the two. The internal `instr_time` struct is the monotonic clock; `TimestampTz` is the wall clock. Crossing them requires explicit conversion. Mirror's `gen_prism.tick: u64` is **monotonic by construction** (it counts message applications, never decreases); `compute_bound.max_wall_time` is wall-clock budget. These are different types; the current `u64` collapse hides it.

**2.5 leap seconds (PG docs §B.7).** PG's `epoch` extraction *does not include leap seconds*. The Unix epoch convention is followed deliberately; users who need TAI (atomic time) get a documented mismatch. The lesson: **the algebra picks a convention and names it**, rather than pretending to handle all conventions transparently.

**2.6 special values (PG docs §8.5.1.5).** `infinity`, `-infinity`, `now`, `today` are recognized literals. `infinity` in a `tstzrange` makes `[now(), infinity)` the canonical "open-ended" range. Mirror's `option(duration)` collapses `none = unbounded`, which is correct but loses the `[lower, infinity)` structure that range types preserve.

**2.7 the calendar weirdness suite.** PG handles Julian/Gregorian transitions, BC dates, the absence of year zero, 24:00:00 as a valid time-of-day (same instant as next-day 00:00:00 for some operations). Most of this is *out of scope* for mirror; it matters only when grammars target calendar surfaces (`@calendar/gregorian`). Naming it as a separate grammar keeps the substrate clean.

---

## 3. Mirror's existing time substrate (inventory)

Three places already carry time concerns; none reference each other:

- **`boot/std/time.mirror`** declares `type tick(u64)` (a point in spectral state) plus `snapshot / timeline / cursor / mutation / delta / replay / fork` and a `@time` grammar with actions for time travel (`enter / restore / browse / compare / replay / fork / step / present`). **Time here is `git log` for graphs.** It is *spectral* time — the substrate's own evolution, not wall-clock time.
- **`boot/std/beam.mirror:25`** declares `type duration(u64)` (BEAM-local; how long a beam took to traverse the prism). A `beam` record carries `emitted: tick, duration: duration`. **Time here is monotonic, runtime-internal, BEAM-coupled.**
- **`boot/std/epistemologic/silicon/compute_bound.mirror:34`** declares `type wall_time = u64` *locally* with the explicit comment: *"Declared locally rather than imported from @beam.duration to keep the silicon carriers from pulling the full BEAM runtime grammar … Design call parked: promote to a shared @time.duration if a second silicon carrier needs the same representation."* **Time here is wall-clock, budget-bearing.**

The substrate is **three u64s pretending to be the same type**. The design call parked it because @beam was the wrong import surface, not because the unification was wrong. The shared `@time.duration` is the right unification surface.

---

## 4. The `@time.duration` design

Concrete grammar sketch. Honours PG's lessons; rewrites them in mirror's substrate vocabulary.

```
grammar @time/duration {
  in @prism
  in @meta
  in @option
  in @epistemologic
  in @epistemologic/property
  in @epistemologic/property/content_addressed
  in @epistemologic/property/monotonicity
  in @epistemologic/property/duration_algebra

  # The base carrier. Non-negative by construction (refract witness):
  # constructors that would produce a negative duration return imperfect.
  # u64 nanoseconds gives ~584 years of range — sufficient for any
  # workload the substrate will host.
  type duration = refract({ nanos: u64 })

  # The two frames PG distinguishes as CLOCK_MONOTONIC vs CLOCK_REALTIME.
  # zoom(T): annotation-only. Same bytes, different declared shape; the
  # type checker refuses to add a wall to a monotonic without going
  # through an explicit convert(_) at a shard boundary.
  type monotonic = zoom(duration)   # gen_prism tick deltas; compute budgets
  type wall      = zoom(duration)   # @time.tick deltas in observed-frame terms

  # PG's interval: (months, days, microseconds) — the calendar-aware
  # form. NOT a duration. Frame-dependent. Lives in a different grammar
  # (@time/interval) and is deliberately not a `duration` subtype.
  # Documented here as a non-goal of this carrier.

  # The five Prism operations on duration:
  focus(d: duration) -> u64 { d.nanos }                       # extract the scalar
  project(d: duration, unit: time_unit) -> precision { \ }    # convert to ms/μs/s/min
  split(d: duration, n: u32) -> [duration] { \ }              # n equal parts; partial if not divisible
  zoom lift(d: u64) -> imperfect                              # u64 ns → duration; succeeds if non-negative
  refract(d: duration) -> oid                                 # content-address; the witness

  # Monoid under +. Associative, commutative, zero-identity.
  # Subtraction is partial: returns imperfect with partial on underflow.
  add(a: duration, b: duration) -> duration { \ }
  sub(a: duration, b: duration) -> imperfect { \ }

  # Multiplication by a non-negative scalar is total; signed scalars
  # are deliberately not defined here (semantics would be ambiguous
  # for a non-negative carrier).
  scale(d: duration, factor: u64) -> duration { \ }

  # The properties this carrier must satisfy.
  requires content_addressed(duration)
  requires monotonicity(monotonic)
  requires duration_algebra(duration)
}

out duration
out monotonic
out wall
out add
out sub
out scale
out @time/duration
```

**Five operations because the algebra has five.** `focus` extracts the scalar; `project` narrows to a unit; `split` distributes; `zoom` annotates the frame; `refract` content-addresses. The shape echoes `compute_bound`'s five-field justification (each axis of resource extraction maps to one Prism op).

**`refract({ nanos: u64 })` not `type duration = u64`.** The refract form is the verified-construction witness — building a `duration` *is* the proof that `nanos` is non-negative. The PG lesson `2.5` says: pick a convention, name it. The convention here is "nanoseconds since the frame's epoch, non-negative."

**`zoom(duration)` for monotonic vs wall.** PG's `CLOCK_MONOTONIC` / `CLOCK_REALTIME` split is the canonical instance of the phantom-types pattern: same bytes, different semantic frame. `zoom(T)` IS the phantom-types verb at the type layer (per `parametric-types-and-fp-heritage.md`); using it here means **adding a `monotonic` to a `wall` is a type error**, not a silent corruption.

**Interval as a separate grammar.** The PG `2.2` lesson is the strongest one — duration arithmetic is *not* associative across frames when calendar arithmetic is in play. The honest move is to refuse to overload `duration + duration` with calendar semantics and put interval into its own grammar (`@time/interval` for the future calendar work). This keeps `@time.duration` total and predictable.

---

## 5. Time-as-substrate maps onto observer-relativity

`@mirror/shard/self` is observer-relative λ₀ (per `2026-05-25-shard-as-observer-relative-lambda-zero.md`). The observer's frame already includes the timezone, the monotonic clock origin, the wall-clock origin, the kernel's CLOCK_BOOTTIME offset. The shard *should* carry the observer's time frame, not as a separate field but as the frame any `wall` duration is annotated against.

The operational consequence: comparing two `wall` durations from different shards is a **cross-observer operation**. PG's lesson `2.1` (timestamptz over timestamp) names this: you cannot compare local-frame values without resolving both to a shared frame. Mirror's analogue:

```
# at a shard boundary, converting between frames requires the shard:
convert(d: wall, from: shard, to: shard) -> imperfect

# within a single shard, durations of the same frame compose freely:
add(a: monotonic, b: monotonic) -> monotonic   # total
add(a: wall, b: monotonic) -> imperfect         # frame mismatch; fails
```

The shard's `parent: option(oid)` chain (the λ₀ fixed point) gives the frame-conversion path: walking up to the common ancestor shard and back down. This is **relativity-of-simultaneity at the substrate** — two shards do not share a `now()`; they share a `compare` operation that resolves through their nearest common ancestor.

The gen_prism's `tick: u64` field (per `boot/std/mirror/runtime/gen_prism.mirror:40`) is already monotonic within a single gen_prism's frame. Promoting it to `tick: monotonic` makes the frame structural rather than conventional.

---

## 6. @epistemologic properties for time

| Property | Justification | PG lesson encoded |
|---|---|---|
| `monotonicity(T)` | Within a frame, time flows forward. Constructing `t2 < t1` from a monotonic source returns `fail`. The `tick` field of `gen_prism` is the load-bearing instance. | §2.4 monotonic/wall split |
| `frame_relativity(T)` | Values are observer-frame-relative; cross-frame comparison requires explicit conversion through `@mirror/shard.self`. The verifier checks that no `compare(a, b)` exists where `a.frame != b.frame` without a conversion in scope. | §2.1 timestamptz |
| `duration_algebra(T)` | T forms a commutative monoid under `+` with zero identity; `-` is partial (underflow → imperfect.partial); `scale` by non-negative scalar is total. Verifies `add` associativity, commutativity, identity, and that `sub` is properly diagnosed. | §2.2 (refused: calendar arithmetic out of scope here) |
| `causality(T)` | Events have partial order; effects follow causes. For any beam with `emitted: tick`, every beam in its `path` has `emitted ≤ self.emitted`. Lamport's logical-clock condition at the type layer. | §2.4 (monotonic clocks are the substrate for causality) |
| `content_addressed(T)` | A duration's oid is the hash of its `{nanos, frame}` content. Required for the substrate-level identity that the kintsugi tournament depends on. (Already in the property set; named here because `duration` must inherit it.) | §2.5 (one convention, named) |

Deferred:

- **`continuous(T)`** — for the real-valued limit case. Mirror is discrete; defer until a numerical grammar needs it.
- **`leap_second_aware(T)`** — only matters at the `@calendar/utc` adapter boundary. Document the substrate convention (no leap seconds in the base `duration`; explicit at the calendar grammar) and defer the property.
- **`dst_aware(T)`** — strictly a property of calendar intervals, not durations. Lives with `@time/interval` when that grammar arrives.

The `halts` property (#74) names a separate concern: load-topology-derived halting needs a *bounded* time budget. `compute_bound.max_wall_time: option(wall_time)` is the carrier; `halts(T)` reads it. Promoting `wall_time` to `wall = zoom(duration)` makes the halting property structurally tied to the frame: a halting argument that mixes monotonic-reductions with wall-time will fail the check.

---

## 7. Connection to landed substrate work

- **`@epistemologic/property/halts` (#74).** The property reads `compute_bound.max_wall_time`. Today that's `option(u64)`; after `@time.duration` it's `option(wall)`. The structural halting argument inherits the frame discipline — the substrate cannot accidentally compare a budget in wall-time to elapsed monotonic reductions.
- **`@epistemologic/silicon/compute_bound` (Mara's #65 α/5, commit `27a3bec`).** The carrier currently declares `type wall_time = u64` with the deferred-promotion comment. Promotion to `wall = zoom(duration)` is a one-line type change at the carrier; downstream consumers (`detect_max`, `admits`, the shard's `compute` field) are unaffected because `zoom(T)` is annotation-only.
- **`@code/beam/eaf`.** BEAM has its own monotonic time (`erlang:monotonic_time/0`, `erlang:system_time/0`) and the conversion is well-documented. The mirror→EAF emit target needs a `from_beam(beam_time: u64) -> monotonic` shim at the grammar's emit boundary. The shim lives in `@code/beam`, not in `@time` — keeping `@time` substrate-pure, isolating the BEAM specifics at the surface that targets BEAM.
- **Kintsugi as Ricci flow.** Ricci flow IS time evolution on a manifold: ∂g/∂t = -2 Ric(g). The kintsugi tournament's iteration parameter is the *flow time* (per `docs/specs/kintsugi-formatter.md` and `2026-05-25` kintsugi-tournament spec). Promoting it to `monotonic` makes the discrete-flow framing structurally honest — the substrate now distinguishes the manifold's geometric time (`monotonic`) from the user's wall-clock observation (`wall`). The Banach contraction argument's iteration bound becomes a `monotonic` quantity verified by `monotonicity(monotonic)`.
- **`@time.tick` (existing `boot/std/time.mirror`).** The spectral-state tick already exists as `tick(u64)`. It is the substrate's *own* monotonic time — every snapshot is one tick further from the root. Promotion: `type tick = monotonic`. The `delta` record's `holonomy: loss` is the discrete Ricci curvature being smoothed; the `replay` is the flow re-evaluated from an earlier `tick`.

---

## 8. Open questions

1. **Does `monotonic + wall` need a third frame `instant`?** PG's `timestamptz` is an instant (UTC absolute); duration is the delta. Mirror's `duration` is the delta; the instant is implicit in the snapshot's `tick: monotonic` field. Decision needed: do we ever need `type instant = refract({ epoch_ns: u64, frame: shard })` as a separate carrier, or is "a snapshot's tick" always sufficient? The substrate-as-source-of-truth principle favours snapshot-tick; cross-shard messaging might force instant.
2. **How do we handle PG's `infinity` literal?** The natural mirror encoding is `option(duration)` with `none = unbounded`. PG distinguishes `+infinity` and `-infinity` in ranges. Since `duration` is non-negative, `-infinity` is structurally inadmissible. Do we need `enum bound = finite(duration) | infinity` to preserve the `[t, infinity)` range structure, or does `option(duration)` suffice for the budget use case?
3. **The `interval` grammar (`@time/interval`) — when does it land?** Calendar arithmetic is a v2.0 concern (no current grammar needs it). Naming it here as the structural separation point is enough for v1.0; explicit refusal to overload `+` for calendar semantics protects `duration` from the PG §2.2 trap until calendar work arrives.
4. **Should `tick(u64)` in `@time.mirror` be promoted to `tick: monotonic` *in this cycle*?** The change is one line. The downstream impact is zero (zoom is annotation-only). The benefit is that every existing time consumer (snapshots, deltas, replays, the gen_prism's tick field) gets the frame discipline for free. Recommend: yes, fold into the same tick.
5. **Cross-shard `compare`: explicit conversion or implicit via shard ancestor walk?** Section 5 sketched a `convert(d, from, to)` boundary action. Open: should the comparison verb (`<`, `==`) on cross-frame durations require the convert call in source, or should the type system insert it automatically by walking the shard ancestry? Explicit is safer (PG's lesson: never lie about the frame); implicit is more ergonomic. Recommend explicit by default; add an `auto_convert` glass tag for grammars that need the ergonomics.
6. **Leap-second policy.** The substrate convention `2.5` says: pick one, name it. Recommend: **`duration` is leap-second-free seconds** (matches Unix epoch, matches BEAM, matches PG `extract(epoch)`); the `@calendar/utc` grammar (future) carries the leap-second-aware path with its own adapter. Document at the grammar header so the convention is discoverable.

---

## 9. References

**PostgreSQL documentation (cited from training; verify section numbers against current docs):**
- §8.5 Date/Time Types — the canonical `timestamp` / `timestamptz` / `date` / `time` / `interval` discussion.
- §8.5.1.4 Interval Input — the `(months, days, microseconds)` triple; the frame-dependence story.
- §8.5.1.5 Special Values — `infinity`, `-infinity`, `now`, `today`, `epoch`.
- §8.17 Range Types — `tstzrange`, `&&`, `@>`, inclusivity bounds; the `multirange` extension (PG ≥ 14).
- §B.7 Date/Time Configuration — `TIMEZONE` GUC; leap-second policy (none).
- PG ≥ 14 internals (`src/include/portability/instr_time.h`) — `CLOCK_MONOTONIC` vs `CLOCK_REALTIME` separation.

**Distributed-systems / formal lineage:**
- Lamport (1978), *Time, Clocks, and the Ordering of Events in a Distributed System* — the partial-order causality framing the `causality(T)` property encodes.
- Lamport (1990s onward), TLA+ time model — discrete time with monotonic action sequences; mirror's tick-as-action-count matches.
- Einstein (1905), *On the Electrodynamics of Moving Bodies* — relativity-of-simultaneity; the deep prior art for shard-relative `now()`.
- ISO 8601 / RFC 3339 — the wire formats. Out of scope for the substrate; the `@code/json` and `@code/postgres` adapters handle them at their boundaries.

**Internal cross-references:**
- `mirror/boot/std/time.mirror` — the existing `@time` grammar.
- `mirror/boot/std/beam.mirror:25` — `type duration(u64)` (BEAM-local).
- `mirror/boot/std/epistemologic/silicon/compute_bound.mirror:34` — `type wall_time = u64` (the parked carrier).
- `mirror/docs/specs/shard-design.md` §3 — `self()` as λ₀; observer-relative resolution.
- `mirror/docs/insights/2026-05-25-shard-as-observer-relative-lambda-zero.md` — frame relativity at the substrate.
- `mirror/docs/insights/2026-05-25-parametric-types-and-fp-heritage.md` — `zoom(T)` / `refract(T)` discipline.
- `~/dev/systemic.engineering/practice/insights/beam-elixir/2026-02-26-mnesia-postgres-sync.md` — the corpus's most thorough working example of `timestamptz` + Lamport-clock causal ordering across BEAM + PG.

---

*Apache-2.0. Time is substrate. PostgreSQL has the lineage; mirror has the algebra; the shared `@time.duration` is the unification surface. Five operations, two frames, three properties, one convention. The substrate picks; the grammar names; the property verifies.*
