# Inference Physics

**Date:** 2026-03-31
**Authors:** Alex Wolf, Reed
**Status:** Design approved, pending implementation
**Touches:** coincidence, conversation (model, check, compile, ffi, parse, resolve, property, runtime, spectral)

---

## Thesis

The model doesn't have a temperature parameter. It has a domain.
The domain determines the physics of inference.

The spectral dimension of a grammar's type graph determines the diffusion
time of inference. Complex domain, high d_s, more paths, slower collapse,
more coherent output. Simple domain, low d_s, fewer paths, faster collapse,
still coherent. The grammar is sub-Turing — Rice's theorem doesn't apply —
so the complexity is decidable. Coincidence computes it from the eigenvalues.

---

## Architecture

```
coincidence          conversation
-----------          ------------
Eigenvalues  ──────> DomainSpectrum ──> DomainComplexity
  (newtype)            (wrapper)          │
  heat_kernel()                           │
  spectral_dimension()                    ▼
  diffusion_time()                    Verified
  temperature_at()                      │ domain + spectrum
                                        │ compile-time ceiling
                                        ▼
                                   DomainActor (ractor)
                                      │ InferenceSchedule
                                      │ runtime narrowing
                                      ▼
                                   decide handler
                                      │ temperature from eigenvalues
                                      │ context scales within ceiling
                                      ▼
                                   provider.infer()
```

Compile-time computes the static d_s and bakes a baseline schedule.
Runtime can narrow it (simpler context = faster) but never widen it
(can't exceed the domain's complexity budget). The compile-time value
is the ceiling.

---

## 1. Eigenvalues newtype (coincidence)

A newtype in coincidence that enforces sorted, non-negative, from-Laplacian.
Only constructable from eigendecomposition — not from arbitrary floats.

```rust
// coincidence::eigenvalue

/// Eigenvalues of a graph Laplacian.
/// Sorted ascending. All non-negative (Laplacian is PSD).
/// Only constructable from eigendecomposition.
pub struct Eigenvalues(Vec<f64>);

impl Eigenvalues {
    pub(crate) fn from_sorted(values: Vec<f64>) -> Self;

    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn as_slice(&self) -> &[f64];

    /// λ₂ — algebraic connectivity. The decide moment.
    pub fn fiedler_value(&self) -> Option<f64>;

    /// λ₃ - λ₂ — sharpness of collapse.
    pub fn eigengap(&self) -> Option<f64>;

    /// K(t) = Σ exp(-λ_k t) — heat kernel at diffusion time t.
    pub fn heat_kernel(&self, t: f64) -> f64;

    /// d_s(t) = -2 d(ln K)/d(ln t) — spectral dimension at scale t.
    pub fn spectral_dimension(&self, t: f64) -> f64;

    /// Scale diffusion time by complexity fraction (0.0–1.0).
    /// complexity_fraction = 1.0 uses full d_s budget.
    pub fn diffusion_time(&self, complexity_fraction: f64) -> f64;

    /// Temperature at diffusion time t, derived from heat kernel.
    pub fn temperature_at(&self, t: f64) -> f64;
}
```

All spectral math lives in coincidence. Conversation holds `Eigenvalues`
and asks questions. Coincidence answers.

---

## 2. DomainSpectrum and DomainComplexity (conversation)

`DomainSpectrum` wraps `Eigenvalues` with domain context. Only constructable
via `Domain::from_grammar()`. The eigenvalues are the single source of truth —
everything is a projection of them.

```rust
// conversation::model

/// Spectral analysis of a domain's type graph.
/// Only constructable internally — eigenvalues come from the Laplacian.
pub struct DomainSpectrum {
    eigenvalues: coincidence::Eigenvalues,
}

impl DomainSpectrum {
    pub(crate) fn new(eigenvalues: Eigenvalues) -> Self;
    pub fn eigenvalues(&self) -> &Eigenvalues;
}

/// Grammars with ≤1 type have no type reference graph.
/// Not "spectrum with zeros" — absence of spectrum.
pub enum DomainComplexity {
    /// No type graph to analyze. Instant collapse.
    Trivial,
    /// Real eigenvalues from real graph.
    Spectrum(DomainSpectrum),
}
```

`Trivial` is the absence of a spectrum, not a spectrum with d_s = 0.
Calling `fiedler_value()` on a trivial grammar is a type error.

---

## 3. Verified carries the spectrum

`Verified` is the compile-time proof wrapper. Verification IS measurement.
The spectrum is computed during `verify()`, not before, not after.

```rust
// conversation::check

pub struct Verified {
    domain: Domain,
    spectrum: DomainComplexity,
}

pub fn verify(domain: Domain) -> Result<Verified, Violations> {
    // existing checks: connectivity, property validation...

    let spectrum = match domain.type_graph_edges() {
        edges if edges.is_empty() => DomainComplexity::Trivial,
        edges => {
            let laplacian = Laplacian::from_adjacency(&vertices, &edges);
            let eigenvalues = laplacian.eigenvalues();
            DomainComplexity::Spectrum(DomainSpectrum::new(eigenvalues))
        }
    };

    Ok(Verified { domain, spectrum })
}

impl Verified {
    pub fn domain(&self) -> &Domain;
    pub fn complexity(&self) -> &DomainComplexity;
}
```

You cannot get a `DomainSpectrum` without passing verification.
The schedule is only available for domains that are proven valid.

The `connected` property check becomes a consequence — if the
spectrum shows multiple components, that's a connectivity violation.
Same eigenvalues, reused.

---

## 4. Runtime consumes the proof

The ractor `DomainActor` receives `Verified` at registration and
extracts the inference schedule.

```rust
// conversation::runtime

pub enum InferenceSchedule {
    /// Trivial domain. No exploration needed. Collapse immediately.
    Immediate,
    /// Heat kernel curve from domain's eigenvalues.
    Diffusion(Eigenvalues),
}
```

The actor's `decide` handler:

```rust
let temperature = match &self.schedule {
    InferenceSchedule::Immediate => 0.0,
    InferenceSchedule::Diffusion(eigenvalues) => {
        // Runtime narrowing: context_complexity ≤ 1.0
        // The domain sets the ceiling. The context scales within it.
        let t = eigenvalues.diffusion_time(context_complexity);
        eigenvalues.temperature_at(t)
    }
};
```

The actor doesn't choose its temperature. It receives it from the
domain's eigenvalues, narrowed by context.

### ODA phases emerge from eigenvalues

The Fiedler value (λ₂) marks the transition from exploration to collapse.
Not an arbitrary phase boundary — a feature of the spectrum.

- Before λ₂ dominates: superposition (observe)
- At λ₂: connectivity forces decision (decide)
- After: committed (act)

The eigengap (λ₃ - λ₂) determines sharpness:
- Large gap → sharp ODA boundaries, fast collapse, decisive
- Small gap → blurry ODA boundaries, slow collapse, exploratory
- Multiple components → independent collapse events

The grammar's topology determines not just the temperature but the
cognitive style of the inference.

---

## 5. The @ai grammar

The existing `@ai` grammar in the garden gains property declarations
and the `schedule` type:

```
grammar @ai {
  type = observation | decision | action | model

  type observation = ref | embedding | signal
  type decision = vector
  type action = generate | route | embed | stop
  type model = local | remote | hybrid
  type schedule = immediate | diffusion

  requires inference_justified

  action decide(observation, schedule) in @rust {
    // spectral-derived temperature, provider call
  }

  action generate(decision) in @rust {
    // token generation with temperature from schedule
  }

  action embed(observation) in @rust {
    // embedding computation
  }

  action route(observation, model) in @beam {
    // model selection dispatch
  }
}
```

`@coincidence` gains:

```
action spectral_dimension(spectrum) in @rust {
  // d_s computation via Eigenvalues
}
```

---

## 6. Action surface redesign

Actions are functions. No body = compile error.

### Concrete actions

```
action decide(observation, schedule) in @rust {
  // body compiles to specified target
}
```

- `in @rust` → NIF, compiled via Cargo, called through Rustler
- `in @beam` → Erlang/Gleam module, loaded via code server
- `in @erlang` → apply/3 dispatch to existing Erlang functions

No body without a target. No target without a body.

### Abstract actions

```
abstract action observe(observable)
```

Signature only. Must be implemented by a grammar that says
`in @cogito`. The compiler rejects a grammar that claims
`in @cogito` but doesn't implement all abstract actions.

### Syntactic sugar

When the type name matches the parameter name:

```
# These are identical:
action decide(observation, schedule)
action decide(observation: observation, schedule: schedule)

# When they differ, name both:
action decide(obs: observation, sched: schedule)
```

Bare identifiers resolve as `name: type` where name equals type.
Type must exist in the grammar's type surface — if `observation`
isn't a declared type, compile error.

---

## 7. Property validation

New builtin property: `inference_justified`.

```rust
// property.rs
"inference_justified" => Some(BuiltinProperty::Registry(inference_justified_check)),
```

Checks:

1. **Domain has a spectrum** — grammar declares inference actions but
   type graph is trivial → violation. Claiming inference with nothing
   to reason about.

2. **Connectivity** — disconnected type graph means independent
   collapse events. Flagged so the developer knows inference will
   fragment.

3. **Finite spectral dimension** — pathological type graphs could
   produce d_s values implying impractical diffusion times. Warns
   above configurable threshold.

Declared in the grammar:

```
grammar @ai {
  requires inference_justified
}
```

Compiler runs it at verification time. If the grammar's type surface
doesn't support coherent inference, compilation fails.

---

## 8. TypeRegistry elimination

Domain absorbs TypeRegistry's remaining surface entirely.
TypeRegistry is deleted.

### New fields on Domain

```rust
pub struct ActionCall {
    pub target: DomainName,
    pub action: ActionName,
    pub args: Vec<TypeName>,
}

pub struct ActionBody {
    pub target: DomainName,  // in @rust, in @beam, etc.
    pub source: String,      // raw body text
}

/// Newtype for action parameter names.
pub struct ParamName(String);

pub struct Action {
    pub name: ActionName,
    pub params: Vec<(ParamName, TypeName)>,
    pub body: Option<ActionBody>,   // None for abstract actions
    pub visibility: Visibility,
}

pub struct Domain {
    name: DomainName,
    types: Vec<TypeDef>,
    actions: Vec<Action>,
    lenses: Vec<DomainName>,      // in @domain declarations
    extends: Vec<DomainName>,     // extends @domain declarations
    calls: Vec<ActionCall>,       // cross-actor @domain.action() calls
    properties: Properties,
}
```

### Migration

1. Add missing fields to Domain (`extends`, `calls`, action body/target)
2. `ContentAddressed` impl on Domain (replaces TypeRegistry's `encoded()`)
3. `Namespace` maps `DomainName → Domain` instead of `String → TypeRegistry`
4. `compile.rs` / `ffi.rs` take `&Domain` directly
5. Property checks take `&Domain`
6. Delete TypeRegistry

The parser is unchanged. `Parse` produces `Prism<AstNode>`.
`Domain::from_grammar()` consumes it. The parser doesn't know about Domain.

---

## 9. Testing strategy

### Coincidence tests

- `Eigenvalues` newtype: construction from Laplacian, sorted invariant,
  non-negative invariant
- `heat_kernel()`, `spectral_dimension()`, `diffusion_time()`,
  `temperature_at()` — pure math, deterministic, property-testable
- Grammar-scale graphs: d_s sensible for small type graphs
  (path≈1, star≈1.5, complete→2)

### Conversation compile-time tests

- `DomainSpectrum::new(eigenvalues)` — verify spectrum computed from
  type references
- `DomainComplexity::Trivial` — grammar with ≤1 type, no spectrum
- `verify()` produces `Verified` carrying the spectrum
- `inference_justified` property: pass for well-formed `@ai` grammars,
  fail for trivial grammar claiming inference
- `abstract action` without downstream implementation → compile error
- `action` without body → parse error
- `action ... in @rust` → correct target recorded in Domain
- Syntactic sugar: `action f(x)` parses as `action f(x: x)`

### Runtime tests

- `InferenceSchedule::Immediate` for trivial domains
- `InferenceSchedule::Diffusion(eigenvalues)` for real domains
- Actor receives schedule from `Verified`, temperature derived
  from eigenvalues
- Runtime narrowing: `context_complexity = 0.1` gives lower
  temperature than `1.0`
- Ceiling holds: no context produces temperature above compile-time bound

### Integration litmus test

`.conv` source → parse → Domain → verify with spectrum → actor boots
with schedule → decide uses temperature from eigenvalues.

---

## Non-goals

- Custom model training on garden grammars (Step 5 from roadmap — future work)
- Tensor product projection for dependent types (future work)
- Actual LLM provider integration (the `provider.infer()` call is an interface,
  not an implementation in this sweep)
- Hot-reloading domain spectrum when grammar changes at runtime

---

## Open questions

None. Design is locked.
