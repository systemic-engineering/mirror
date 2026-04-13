# Prism Cascade: Mirror + Fate + spectral-db Migration

**Date:** 2026-04-11

**Status:** Draft

**Approach:** Sequential (B) — Cargo.toml alignment first, then Fate, then coincidence, then Mirror, then spectral-db, then prism core cleanup.

---

## Context

The prism crate has been redesigned:

- `Imperfect<T, E, L>` replaces `Luminosity<V, E>` — three variants: `Success(T)`, `Partial(T, L)`, `Failure(E)`
- `Loss` trait parameterizes loss type. `ShannonLoss` is default.
- `Beam` trait: `tick` replaces `advance`, `smap` is the semifunctor map, `next` is lossless shorthand.
- `Prism` trait: three operations (focus, project, refract). Split/zoom removed — user-space `smap`.
- `Op` enum: three variants (Focus, Project, Refract).
- Workspace: `prism/imperfect` (standalone) + `prism/core` (depends on imperfect).

144 tests pass. The foundation is stable. This spec covers the first consumers.

---

## Phase 0: Cargo.toml Alignment

Prism is now a workspace (`prism/imperfect` + `prism/core`). Every consumer that depended on `prism = { path = "../prism" }` fails because the workspace root is a virtual manifest, not a package.

**Fix:** All consumers update to `prism = { package = "prism-core", path = "../prism/core" }`. The `package` alias preserves the Rust import name as `prism` — no source changes needed for the Cargo.toml fix alone.

**Consumers requiring update:**

| Crate | Blocks |
|-------|--------|
| coincidence | mirror, spectral-db |
| fate | mirror |
| mirror | spectral-db, spectral |
| spectral-db | spectral |
| spectral | — |
| lens | spectral |
| cosmos | — |
| conversation | — |
| metal-runtime | — |

**Build order (must compile in this order):**

1. coincidence + fate (no cross-dependency, parallel-safe)
2. mirror (depends on coincidence + fate)
3. spectral-db + lens (depend on mirror)
4. spectral (depends on all)

Each crate may also have source-level compilation errors from the new Prism trait shape (different associated types, missing methods, renamed types). Phase 0 fixes Cargo.toml only. Subsequent phases fix source.

**Discovery:** Fate has 74 compilation errors against the new prism. It must compile before mirror can. The original plan had Fate as Phase 2 after Mirror — reversed. Fate is now Phase 1.

---

## Phase 1: Fate

Mirror has two Prism implementations (Shatter, ASTPrism) and its own Beam type.

### Shatter (mirror_runtime.rs)

- Remove `split` and `zoom` methods. Both are already no-ops (`// TBD`).
- Keep `focus` (Form → MirrorData), `project` (MirrorData → MirrorFragment), `refract` (MirrorFragment → Shatter).
- Align associated types to `Input/Focused/Projected/Refracted`. Remove `Part`/`Crystal` if present.

### ASTPrism (ast_prism.rs)

- Same treatment. Remove split/zoom. Keep focus/project/refract.

### TraceBeam (new, in mirror)

Mirror implements TraceBeam as its compilation beam:

```rust
pub struct TraceBeam<In, Out, E = Infallible, L: Loss = ShannonLoss> {
    inner: PureBeam<In, Out, E, L>,
    trace: Trace,
}
```

- Implements `Beam` trait by delegating to `inner` and recording a `Step` per `tick`/`next`/`smap`.
- Mirror's current beam metadata (path, loss, precision, recovered, stage) maps to Trace steps.
- Lives in mirror. Moves to prism/core later if the shape proves general.
- PureBeam is for production speed (ants). TraceBeam is for compilation observability (watching the compiler think).

### Type Migration

| Old | New |
|-----|-----|
| `Luminosity::Radiant(v)` | `Imperfect::Success(v)` |
| `Luminosity::Dimmed(v, loss)` | `Imperfect::Partial(v, loss)` |
| `Luminosity::Dark(e)` | `Imperfect::Failure(e)` |
| `beam.advance(luminosity)` | `beam.tick(imperfect)` |
| `beam.is_light()` | `beam.is_ok()` |
| `beam.is_dark()` | `beam.is_err()` |
| `Op::Split` / `Op::Zoom` | Removed |

### Extension Rename

`.conv` → `.mirror` where applicable in mirror's own file handling.

### Tests

Mirror has 810+ tests. Update all tests touching Shatter, ASTPrism, Beam, Op, Luminosity. Majority untouched.

---

## Phase 1b: Coincidence

Coincidence depends on prism for spectral hashing. Cargo.toml fix in Phase 0. Source-level changes:

- Update any `Luminosity` → `Imperfect` references.
- Update any `Beam` method calls (advance → tick, is_light → is_ok, etc).
- `SpectralHash<N>` pipeline: verify it compiles against new Beam trait.

Coincidence must compile before mirror (mirror depends on it).

---

## Phase 2: Mirror

---

## Phase 3: spectral-db + remaining consumers

### Type Relocations

| Type | From | To |
|------|------|----|
| `SpectralOid` | `prism/core/src/spectral_oid.rs` | `spectral-db/src/spectral_oid.rs` |
| `Precision` | `prism/core/src/precision.rs` | `spectral-db/src/precision.rs` |
| `Pressure` | Already in `spectral-db/src/pressure.rs` | Stays |
| `ShannonLoss` | `imperfect` crate | Stays (re-exported through prism core) |

### Precision as smap

Precision zoom is a user-space smap with measurable loss:

```rust
spectral_oid_beam.smap(|oid| {
    let truncated = oid.truncate(precision);
    let loss = ShannonLoss::new(oid.information_lost(precision));
    Imperfect::Partial(truncated, loss)
})
```

The precision cut IS the loss. Resolution = how much you keep. Loss = how much you don't.

### Schema

- `db.conv` → `db.mirror`. Schema grammar uses three operations.
- Schema parsing calls mirror's updated pipeline.

### Query Results

Upgrade from `Result<T, E>` / `Option<T>` to `Imperfect<T, E, L>` where natural:

- Exact match → `Imperfect::Success(result)`
- Approximate match → `Imperfect::Partial(result, loss)`
- No match → `Imperfect::Failure(err)`

### Convergence

- Fiedler bisection / crystallizer lens: natural Partial states.
- Convergence detection: two spectra match at a given Precision → settled.
- Loss trait for convergence metrics instead of ad-hoc f64 comparisons.

---

## Phase 4: Prism Core Cleanup

After all consumers migrated:

- Remove `spectral_oid.rs` from prism/core
- Remove `precision.rs` from prism/core
- Core retains: Beam, Prism, Op, Trace, Oid, Connection, ContentAddressed
- Re-exports: Imperfect, Loss, ShannonLoss from imperfect crate
- Zero domain-specific types remain

---

## Dependency Order

```
imperfect (done, 44 tests)
    ↓
prism/core (done, 94 tests + 6 integration)
    ↓
Phase 0: Cargo.toml alignment (all consumers)
    ↓
Phase 1: fate + coincidence (parallel-safe, no cross-dep)
    ↓
Phase 2: mirror (depends on fate + coincidence)
    ↓
Phase 3: spectral-db + lens + spectral (after mirror)
    ↓
Phase 4: prism core cleanup (after all consumers)
```

Fate blocks mirror (mirror depends on fate). Coincidence blocks mirror (mirror depends on coincidence). Both are leaf dependencies — fix them first, in parallel.

---

## Success Criteria

- All existing tests pass after each phase (mirror 810+, fate 50, spectral-db existing suite).
- No references to Luminosity, advance, Op::Split, Op::Zoom in migrated crates.
- TraceBeam in mirror records observable compilation steps.
- SpectralOid/Precision live in spectral-db, not prism.
- Prism core has zero domain-specific types.
- Fate models produce identical selections post-migration (weights hold or retrained).
