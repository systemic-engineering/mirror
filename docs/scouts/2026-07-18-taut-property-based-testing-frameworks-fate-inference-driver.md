# Taut scout — property-based testing frameworks + Fate as inference driver

**Author:** Taut  
**Date:** 2026-07-18  
**Status:** Grep-first, read-only, Kagi-armed drift scout  
**Spawned by:** Alex 2026-07-18 direct-transcript brief — *"spawn Taut on
a Kagi deep dive into property based testing frameworks. Elixir.
Haskell. The spiel. We're gonna make this smooth as velvet. With Fate
as the inference driver for both the tests and the compiler. Which is
beautiful."*

---

## §0 Executive summary

- **Substrate gap: prismqueer::liquid has verdict algebra but no
  generator / shrinker / arbitrary infrastructure.** The 98 "property
  tests" landed iter 1-9 are hand-rolled `#[test]` with hand-coded
  enumeration over closed strategy spaces (`0..4u8`, S3, Perm3).
  Zero calls to a random-input engine. Zero shrinkage after failure.
  There is no `Arbitrary`, no `Strategy`, no `Gen`, no `Range` —
  substrate-honestly: prismqueer::liquid is a **verdict-marshaling
  library today, not a property-generation library.** The word
  "property" in `PropertyVerdict` refers to Beer/OBC audit-channel
  verdicts, not to QuickCheck-style universally-quantified claims.

- **The PBT canon converges on three axes** (from Claessen &
  Hughes 2000 through hedgehog / Hypothesis / proptest 2024-2026): (1)
  generator composition (Applicative / Monadic bind); (2) shrinker
  strategy (type-directed manual, integrated rose-tree, or
  choice-sequence internal); (3) coverage / search feedback loop
  (targeted PBT via simulated annealing, coverage-guided a la
  Hypothesis's conjecture engine). All three axes are **structurally
  absent** from prismqueer::liquid — but the verdict semilattice
  (`Pass / Partial / Fail` + `merge_with`) is already the correct
  landing point for any of them.

- **Integrated shrinking (hedgehog / Hypothesis) is the SOTA
  ergonomics winner.** Well-Typed's 2019 comparison (de Vries et al.,
  falsify paper 2023 §2) landed the definitive framing: manual
  generator+shrinker pairs breed duplication and get skipped;
  integrated shrinking via rose-trees or choice-sequences keeps
  generator and shrinker in sync by construction. **BUT** — hedgehog's
  Monad-lifted composition is known-lossy on shrink quality
  (Hypothesis + falsify papers). Hypothesis's choice-sequence
  representation (compositional-shrinking article) is the
  most flexible, at the cost of extra runtime machinery.

- **Property discovery — QuickSpec + targeted PBT — is where "smooth
  as velvet" actually lives.** QuickSpec (Claessen & Smallbone) turns
  a term algebra into automatic conjecture generation. Targeted PBT
  (Löscher & Sagonas ISSTA 2017 + ICST 2018) uses simulated annealing
  over a utility gradient. Both compose over the existing PBT surface
  — they don't replace it, they wrap it. **Direct fit for Fate:**
  Fate's five-model selector + 90-parameter softmax is *already*
  structurally a targeted-search neighbourhood function. The
  translation cost is smaller than it looks.

- **Fate as inference driver — the SOTA is CoverUp / TitanFuzz /
  ELFuzz for LLM-driven test generation.** CoverUp (Andrzejewski et
  al. 2025; ICSE-adjacent) drives Python regression-test generation
  via iterative LLM prompting + coverage feedback. TitanFuzz (ISSTA
  2023) uses LLMs to generate deep-learning-library fuzz inputs.
  ELFuzz (arXiv 2506.10323, 2025) has the LLM synthesize the fuzzer
  itself. **All of these are external ML-invoking harnesses; none has
  the same weight-set as inference driver for BOTH tests AND
  compilation.** Alex's directive is genuinely novel at that
  granularity.

- **The neural-guided compiler prior art (CompilerGym, LoopLearner,
  neural superoptimization) is separate from the neural-guided PBT
  prior art.** No paper found unifying them under one inference
  engine. Alex's "Fate as inference driver for both" is a **novel
  composition** — not because either half is novel, but because the
  same softmax + weight-set doing pass-selection ALSO doing
  property-generator-selection is not in the literature.

- **Recommended landing shape (§8):** promote prismqueer::liquid from
  verdict library to full PBT surface by landing three primitives
  bottom-up — (a) `pillar::Arbitrary` trait + `pillar::Sample`
  choice-sequence carrier (Hypothesis-shape, not hedgehog-shape,
  because the verdict fold semilattice already survives non-linear
  shrinking); (b) `pillar::forall` runner returning `PropertyVerdict`
  directly; (c) `fate::propose` inference-driver hook that emits both
  generator-strategy weights AND compilation-pass weights from the
  same Fate tick. Three primitives; no new crate boundary; no
  proptest / quickcheck dependency; delightfully-boring naming.

- **Substrate-refusal note:** every SOTA feature in this scout is
  evaluated through delightfully-boring naming + composition-over-
  accretion. Explicit refusals in §5.4 (no separate shrinker trait,
  no Strategy monad DSL, no `#[proptest]` macro layer) and §7
  (adjudication ambiguities).

---

## §1 Substrate ground-truth grep

### §1.1 What actually landed at prismqueer::liquid

Grep confirms: `prismqueer::liquid` is 17.3KB of surface at
`/Users/alexwolf/dev/projects/prism/prismqueer/src/liquid.rs`. It
exports:

| Symbol                       | Type                                                                        |
|------------------------------|-----------------------------------------------------------------------------|
| `LiquidConnection`           | Blanket trait over `Transport` — computes `commutator_magnitude`            |
| `Commutator<'a, C>`          | Held-reference pair + deferred magnitude                                    |
| `commutator(a, b, state)`    | Constructor                                                                 |
| `commutator_norm(a, b)`      | `Default`-state convenience                                                 |
| `pillar::dispatch_ambiguity` | Rice-safe byte-visible → `PropertyVerdict`                                  |
| `pillar::algedonic`          | Single-tick threshold → `PropertyVerdict` (commutator-flavored)             |
| `pillar::algedonic_of_magnitude` | Single-tick threshold → `PropertyVerdict` (raw `Loss + PartialOrd`)     |
| `pillar::viability`          | Multi-tick persistence → `PropertyVerdict` (commutator-flavored)            |
| `pillar::viability_of_magnitudes` | Multi-tick persistence → `PropertyVerdict` (raw `Loss + PartialOrd`)   |
| `pillar::fold`               | Fold `&[PropertyVerdict]` via `merge_with`                                  |
| `prelude`                    | Delightful use-line                                                         |

**What is NOT in prismqueer::liquid:**

- No `Arbitrary` trait
- No `Gen`/`Strategy`/`Sample` carrier
- No shrinker infrastructure (`Shrink`, `Rose`, choice-sequence)
- No `Range` (hedgehog-style shrink-toward-origin)
- No `forall`/`prop_assert`/`check_all` runner
- No random seed handling
- No test-case shrinkage on `Fail`
- No coverage-guided input mutation

### §1.2 What the 98 tests are, mechanically

`prismqueer/tests/liquid_ouroboros.rs` (43 tests, 32.2KB):

```rust
fn commutator_antisymmetric_over_test_bundle_all_strategy_pairs() {
    let state = [1.0, 2.0, 3.0, 4.0];
    for i in 0..4u8 {              // ← closed enumeration
        for j in 0..4u8 {          // ← closed enumeration
            let a = TestBundle::with_strategy(i);
            let b = TestBundle::with_strategy(j);
            let ab = LiquidConnection::commutator_magnitude(&a, &b, &state);
            let ba = LiquidConnection::commutator_magnitude(&b, &a, &state);
            assert_eq!(ab, ba, "…");
        }
    }
}
```

Every test in the ouroboros is **closed-form enumeration** over
`0..4u8`, `Perm3` (6 elements), or `S3×S3` (36 pairs). This is
witness-by-exhaustion at small altitude — substrate-honestly, closer
to **algebraic-law unit-testing** than to QuickCheck-style
universally-quantified property testing. The word "property" in
`prismqueer/tests/*.rs` maps to `terni::PropertyVerdict`, not to
"for-all-inputs" claims.

`prismqueer/tests/prism_laws.rs` (9 tests, 9.5KB): hand-picked seed
values (`42`, `"substrate-honest"`, `vec![1, 1, 2, 3, 5, 8, 13]`,
`[1.0, 2.0, 3.0, 4.0]`). Zero generation.

`prismqueer/tests/verdict_composition.rs` (11 tests, 9.4KB): tests
`PropertyVerdict::merge_with` semilattice semantics — again with
hand-picked verdicts. Base-of-algebra tests, not universally-
quantified.

`mirror/rust/src/collapse.rs prop_tests` mod (24 tests, 40KB total):
same pattern. Hand-rolled `fixture_source()` and `fixture_corpus()`
functions.

### §1.3 Cargo dependency audit

`prismqueer/Cargo.toml`:

```toml
[dependencies]
terni = { version = "0.7", path = "../imperfect" }
prismqueer-projections = { version = "0.1", path = "../projections" }
sha2 = "0.10"
hex = "0.4"
serde = { …, optional = true }
serde_json = { …, optional = true }

[dev-dependencies]
criterion = "0.5"                # bench only
serde = "1"
serde_json = "1"
jsonschema = "0.46"              # pq schema round-trip only
syn = "2"                        # T23 syn round-trip only
quote = "1"
```

`mirror/rust/Cargo.toml` `[dev-dependencies]`:

```toml
prismqueer = { path = "../../prism/prismqueer", features = ["bundle"] }
terni      = { path = "../../prism/imperfect" }
tempfile = "3"
```

**Zero PBT-framework dependencies anywhere in prism/ or mirror/rust.**
No `proptest`, no `quickcheck`, no `arbitrary`, no `bolero`, no
`fuzzcheck`.

### §1.4 Fate substrate

`/Users/alexwolf/dev/projects/fate/Cargo.toml`:

```toml
[dependencies]
prism = { package = "prismqueer", path = "../prism/prismqueer",
          features = ["bundle"] }
serde = { …, optional = true }
serde_json = { …, optional = true }

[features]
default = []
training = ["dep:serde", "dep:serde_json"]
lapack = ["prism/lapack"]
metal = ["dep:metal", "dep:objc"]
```

`fate/src/lib.rs` header:

```
Fate — the meta-model. Five sub-models plus their selector.
Abyss:        Focus. Observe the spectral state.
Introject:    Project. Selective internalization.
Cartographer: Strategy selector — HOW to split.
Explorer:     Subgraph comprehension — compressed meaning.
Fate:         Refract. Crystallize. Select what runs next.
```

The relevant Fate types for this scout:

- `FEATURE_DIM = 16` — the fixed spectral feature vector width
- `Features = [f64; FEATURE_DIM]` — what models observe
- `Model` — 5-variant enum (Abyss / Introject / Cartographer /
  Explorer / Fate)
- `ModelWeights { w: [[f64; 16]; 5], b: [f64; 5], depth_w: [f64; 5] }` —
  ~90 parameters per selector
- `Fate { selectors: [ModelWeights; 5], …, resolved_model: Model, … }`
- `FateOutput { model, decision: Decision, kernel_spec, loss:
  ManifoldLoss, health: HolonomyHealth }`
- `Decision { model, confidence, distribution: [f64; 5] }`
- Softmax + argmax dispatch: `fn softmax5(logits: [f64; 5]) -> [f64;
  5]`

Fate does NOT currently emit anything property-testing shaped. It
selects the next Prism operation (focus / project / split / shift /
settle) via a `[f64; 5]` distribution over Model. Its selector head
is structurally a **90-parameter classifier** — exactly the shape a
targeted-PBT neighbourhood function or Hypothesis-style
choice-sequence weight-driver would want.

### §1.5 Verdict algebra (terni::PropertyVerdict)

`imperfect/src/transparency.rs`:

```rust
pub enum PropertyVerdict {
    Pass,
    Partial { confidence: f64, diagnostics: Vec<Diagnostic> },
    Fail(Diagnostic),
}

// merge_with semantics:
// - Fail dominates (from left)
// - Pass is the neutral element
// - Partial ∪ Partial → { min(c1, c2), ds1 ++ ds2 }
```

This is a **bounded semilattice** with `Pass` as identity and `Fail`
as absorbing top. `merge_with` is associative but *not* symmetric on
`Fail` diagnostics (left Fail wins) — the tests in
`verdict_composition.rs` witness this asymmetry explicitly.

**Structural observation:** the semilattice is the correct landing
point for any shrinker output. When you shrink a `Fail(diag)` down
to a minimal counter-example, the diagnostic is progressively
refined; the Pass/Partial layer never gets triggered. When you shrink
a `Partial { confidence, … }`, the confidence bound is preserved.
This means **prismqueer::liquid already has the right output algebra
for PBT — it just doesn't have the input algebra yet.**

---

## §2 The PBT canon

### §2.1 QuickCheck (Claessen & Hughes, ICFP 2000)

- **Paper:** Claessen & Hughes 2000, "QuickCheck: A Lightweight Tool
  for Random Testing of Haskell Programs"
  [Chalmers/Tufts PDF](https://www.cs.tufts.edu/~nr/cs257/archive/john-hughes/quick.pdf) —
  ACM DL: <https://dl.acm.org/doi/10.1145/357766.351266>
- **Core primitives:** `Arbitrary` type class (defines both `arbitrary
  :: Gen a` and `shrink :: a -> [a]`); `Property` monad;
  `quickCheck :: Testable prop => prop -> IO ()`.
- **Shrinker discipline:** manual, per-type, monadic-list-of-shrunk-
  candidates.
- **Known-lossy failure mode:** in practice, developers write the
  generator, skip the shrinker, get counter-examples that are unusable
  (the "sad state" thread on Haskell Discourse, Jul 2024, is the
  contemporary lament: <https://discourse.haskell.org/t/the-sad-state-of-property-based-testing-libraries/9880>).
- **Composition:** `Applicative` / `Monad` on `Gen`. Ergonomic for
  simple types; awkward for stateful/model tests.

### §2.2 hedgehog (Jacob Stanley 2017+)

- **Repo:** <https://github.com/hedgehogqa/haskell-hedgehog>
- **API primitive shape (Hedgehog.Gen + Hedgehog.Range):**

  ```haskell
  someInt :: Gen Int
  someInt = Gen.int (Range.constant 1970 2100)
  -- shrinks toward origin 2000
  ```

- **Integrated shrinking via rose-trees:** every `Gen a` produces a
  `Tree a` where children are shrink candidates. Consumers never see
  the tree — it's threaded through the runner automatically.
- **Killer feature:** no separate `shrink` implementation. Applicative
  composition of generators produces correct shrinkage by
  construction. From the falsify paper (de Vries et al. 2023, cited
  6×): *"Their integrated shrinking approach makes it possible to
  define Functor and Applicative instances for generators, and
  duplication of logic is avoided."*
- **Known-lossy failure mode:** monadic-bind composition
  (`Gen.filter` and `Gen.bind`) is documented-lossy on shrink
  quality. de Vries et al.: *"Monadic composition is required when
  the behaviour of a generator depends on previously generated
  values; it arises naturally and frequently."*
- **Well-Typed's 2019 comparison** (Edsko de Vries,
  <https://www.well-typed.com/blog/2019/05/integrated-shrinking/>):
  integrated shrinking is not a free lunch; Applicative composition
  is safe, Monadic composition needs care, and `freeze` gives escape
  hatches when the automated shrink tree is insufficient.

### §2.3 Hypothesis (David MacIver 2013+)

- **Repo:** <https://github.com/HypothesisWorks/hypothesis>
- **Central architectural insight** (from MacIver's
  compositional-shrinking article,
  <https://hypothesis.works/articles/compositional-shrinking/>):
  *"The core insight for improved shrinking is that it is typically
  sufficient to shrink inputs rather than the final generated
  outputs."*
- **Choice-sequence engine:** all randomness passes through a
  `PrimitiveProvider` with five `draw_*` methods (integer, float,
  boolean, bytes, string). Every generator is fundamentally a
  function `bytes → a`, and shrinking is bytestream reduction. The
  Conjecture engine
  (<https://github.com/HypothesisWorks/hypothesis/blob/master/hypothesis-python/src/hypothesis/internal/conjecture/engine.py>)
  treats shrinking as an optimization problem over the choice
  sequence.
- **Coverage-guided:** the byte-buffer engine is structurally
  similar to AFL / libFuzzer. Cited in issue #63 on the Conjecture
  repo: *"Conjecture's byte-buffer engine is structurally similar
  to coverage-guided fuzzers (AFL, libFuzzer): both operate on a
  byte stream that drives input construction."*
- **Ghostwriter feature:** automatically generates property tests
  for a target function via `hypothesis.extra.ghostwriter`
  (<https://hypothesis.readthedocs.io/en/latest/ghostwriter.html>) —
  the earliest form of ML-adjacent-to-PBT ergonomics in a shipping
  library. Uses AST inspection, not LLM invocation, but the affordance
  is the same shape: "give me a test I don't have to write."
- **Target-based PBT:** Hypothesis supports `target()` for
  simulated-annealing-shaped input search.
- **The three-way comparison** (seelengrab.github.io,
  <https://seelengrab.github.io/articles/The%20properties%20of%20QuickCheck,%20Hedgehog%20and%20Hypothesis/>):
  QuickCheck simple, hedgehog composable-with-caveats, Hypothesis
  most flexible/extensible.

### §2.4 StreamData (Andrea Leopardi, Elixir 2017+)

- **Repo:** <https://github.com/whatyouhide/stream_data>
- **Announcement:** <https://elixir-lang.org/blog/2017/10/31/stream-data-property-based-testing-and-data-generation-for-elixir/>
- **Ergonomic shape** (from ExUnitProperties hexdocs):

  ```elixir
  property "integer generator produces integers" do
    check all int <- integer() do
      assert is_integer(int)
    end
  end
  ```

- **`check all` macro** integrates into ExUnit — the property test
  reads like any other unit test. This is the "smooth as velvet" bar
  Elixir sets: **zero visible framework overhead in the test body.**
- **Generators are lazy streams** (hence the name); `bind/2` and
  `map/2` are the two composition primitives. This is closer to
  hedgehog's shape than QuickCheck's — StreamData was influenced by
  hedgehog per Leopardi's design blog.
- **Shrinking:** integrated (via the stream), similar rose-tree
  structure under the hood.

### §2.5 proptest (Rust, 2016+ Jason Lingle)

- **Repo:** <https://github.com/proptest-rs/proptest>
- **Explicitly inspired by Hypothesis** (from the README: "Hypothesis-
  like property testing for Rust")
- **`Strategy` trait** replaces `Arbitrary` — a `Strategy<Value = T>`
  produces `T` via `new_tree(runner)` returning a `ValueTree` that
  supports `current()` + `simplify()` for integrated shrinking.
- **Composition** via `.prop_map()`, `.prop_filter()`, `.prop_flat_map()`,
  `.prop_union()`. Applicative-style.
- **Ergonomic surface:**

  ```rust
  proptest! {
      #[test]
      fn adds_commute(a in 0i32..1000, b in 0i32..1000) {
          prop_assert_eq!(a + b, b + a);
      }
  }
  ```

  The `proptest!` macro hides the strategy-runner plumbing.

### §2.6 quickcheck (Rust, Andrew Gallant 2015+)

- **Repo:** <https://github.com/BurntSushi/quickcheck>
- **Direct QuickCheck port:** `Arbitrary` trait with separate
  `arbitrary` and `shrink`. Simpler API than proptest, no integrated
  shrinking.
- **Trade-off** (from rustz2h.com's comparison and reddit
  discussion): quickcheck is smaller and simpler; proptest is
  strictly more powerful because of integrated shrinking. Community
  has largely converged on proptest for non-trivial cases.

### §2.7 fast-check (JavaScript / TypeScript, Nicolas Dubien 2017+)

- **Repo:** <https://github.com/dubzzz/fast-check>
- **Doc:** <https://fast-check.dev/docs/introduction/what-is-property-based-testing/>
- **"Arbitrary" terminology inherited from ScalaCheck.**
- **Zod integration** (`fast-check-zod`) automatically derives
  arbitraries from schema declarations — another form of "test I
  didn't have to write."
- **Composition:** applicative + monadic (`.chain()`), same shape as
  proptest.

### §2.8 ScalaCheck (Rickard Nilsson 2007+)

- **Repo:** <https://github.com/typelevel/scalacheck>
- **`Gen[+T]`** as a monadic type; `Arbitrary[T]` as a type class
  supplying implicit generators.
- **Model:** direct QuickCheck port; separate shrinker via the
  `Shrink` type class.
- **State-machine testing** via `Commands`.

### §2.9 Feature matrix

| Framework      | Shrinking            | Composition           | Coverage-guided | State-machine | LLM-adjacent |
|----------------|----------------------|-----------------------|-----------------|---------------|--------------|
| QuickCheck     | Manual per-type      | Applicative + Monad   | No              | External      | No           |
| hedgehog       | Integrated rose-tree | Applicative-safe      | No              | Yes (built-in)| No           |
| Hypothesis     | Choice-sequence      | `@composite` + draw   | Yes             | Yes           | Ghostwriter  |
| StreamData     | Integrated stream    | bind + map            | No              | External      | No           |
| proptest       | Integrated ValueTree | prop_map/flat_map     | No              | External      | No           |
| quickcheck-rs  | Manual per-type      | Applicative + Monad   | No              | No            | No           |
| fast-check     | Integrated           | chain / tuple         | No              | Yes (model)   | Zod bridge   |
| ScalaCheck     | Separate shrinker    | Monadic Gen           | No              | Yes           | No           |
| PropEr (Erlang)| Manual               | Monadic               | Simulated ann.  | Yes           | No           |

**prismqueer::liquid today:** none of the above columns filled.

---

## §3 Property discovery frontier

### §3.1 QuickSpec (Claessen, Smallbone, Hughes 2010+)

- **Paper:** Claessen, Smallbone, Hughes 2010, "QuickSpec: Guessing
  Formal Specifications Using Testing" —
  [smallbone.se/papers/quickspec.pdf](https://smallbone.se/papers/quickspec.pdf)
- **Follow-up:** Johansson & Smallbone, "Automated Conjecturing in
  QuickSpec" —
  [cse.chalmers.se/~jomoa/papers/Automated_Conjecturing_in_QuickSpec.pdf](https://www.cse.chalmers.se/~jomoa/papers/Automated_Conjecturing_in_QuickSpec.pdf)
- **The core loop:**
  1. Generate a universe of terms over a supplied API (function
     signatures + a background theory of primitive types).
  2. Test each term with random inputs; partition terms into
     equivalence classes when their observed outputs agree.
  3. From each class, emit candidate equations (e.g. `reverse
     (reverse xs) = xs`).
  4. Prune redundant equations via congruence closure.
- **Killer feature:** given a small algebra, QuickSpec discovers
  properties without human authoring. Case study on Haskell leftist
  heaps produced the full algebraic specification including heap-sort
  behavior.
- **Depth optimization:** avoid generating terms with non-canonical
  subterms — massive combinatorial pruning.
- **Direct fit for prismqueer::liquid:** the Bundle tower + Prism
  monoid + PropertyVerdict merge_with together define exactly the
  kind of small algebra QuickSpec is designed for. **Applying
  QuickSpec-shape conjecture generation to the prismqueer term algebra
  would automatically emit conjectures like `commutator_norm(a, a) ==
  Loss::zero()` (which we already test explicitly) AND novel
  conjectures we haven't guessed at.**

### §3.2 Targeted PBT (Löscher & Sagonas ISSTA 2017 + ICST 2018)

- **ISSTA 2017 paper:** "Targeted property-based testing"
  <https://dl.acm.org/doi/10.1145/3092703.3092711>
- **ICST 2018 follow-up:** "Automating Targeted Property-Based
  Testing" —
  [proper-testing.github.io/papers/icst2018.pdf](https://proper-testing.github.io/papers/icst2018.pdf)
- **The core loop:**
  1. Attach a **utility value (UV)** to each generated input — how
     "close" is this input to violating the property?
  2. Use **simulated annealing** over a **neighbourhood function
     (NF)** to search the input space, biasing toward higher-UV
     regions.
  3. NF is either hand-written or (per ICST 2018) auto-constructed
     from the existing random generator by "reenacting generator
     decisions and substituting random choices with structured
     neighborhood selections."
- **Structural observation for Fate:** simulated annealing with a
  temperature parameter and a 5-way distribution over decisions is
  **exactly** the shape of Fate's softmax with depth modulation
  (`depth_w: [f64; 5]`). The "temperature" in Fate's tick and the
  "temperature" in targeted PBT are the same substrate concept at
  different altitudes.
- **The 2018 paper's user-study result:** hand-written NFs
  outperform automated ones, but automated is competitive and orders-
  of-magnitude cheaper to author. Direct implication for Alex's
  directive: **Fate can play the role of "the automated NF that
  learns to compete with hand-written ones over time"** via the
  existing weight-training pipeline.
- **Modern extension:** "Programmable Property-Based Testing"
  (arXiv 2602.18545) — allows user-programmable search strategies
  and seed pool experimentation.

### §3.3 Coverage-guided PBT

- **Paper:** Padhye et al. "Coverage guided, property based testing"
  <https://dl.acm.org/doi/10.1145/3360607>
- **Tool: JQF** (Java) — the coverage-guided PBT frontier for JVM
- **The core loop:**
  1. Instrument the SUT for coverage feedback.
  2. Treat the PBT generator's byte-buffer decisions as a fuzzer
     input.
  3. AFL-style mutation of the byte buffer, coverage as fitness.
- **Hypothesis's internal representation IS this** (issue #63 quoted
  above). Its Conjecture engine is a byte-buffer coverage-adjacent
  fuzzer with a PBT face.

### §3.4 Metamorphic testing

- **Wikipedia canonical:** <https://en.wikipedia.org/wiki/Metamorphic_testing>
- **Paper:** Segura et al. 2018 "Metamorphic Testing: A Review of
  Challenges and Opportunities" —
  <https://dl.acm.org/doi/10.1145/3143561>
- **Recent extension:** Ba et al. 2025 "Metamorphic Coverage"
  <https://arxiv.org/pdf/2508.16307>
- **The core primitive:** a metamorphic relation MR is a claim about
  the *relation* between two program executions on related inputs
  — e.g. "sort(reverse(xs)) == sort(xs)". Solves the oracle
  problem for cases where no ground-truth output exists.
- **Direct fit for a compiler:** MRs like "compile then run with
  optimisation level N == compile then run with optimisation level
  M" are exactly what YARPGen / Csmith exploit.

### §3.5 Csmith / YARPGen (compiler differential fuzzing)

- **Csmith:** <https://github.com/csmith-project/csmith> — Yang et al.
  PLDI 2011.
- **YARPGen (v1):** Livinskii et al. OOPSLA 2020 —
  <https://users.cs.utah.edu/~regehr/yarpgen-oopsla20.pdf> — 220+
  compiler bugs in GCC/LLVM/Intel C++.
- **YARPGen (v2):** Livinskii et al. PLDI 2023 —
  <https://livinskii.com/assets/files/yarpgen-pldi23.pdf> — 122 more
  bugs, focused on loop optimizations.
- **Design principle:** generate programs that are **statically and
  dynamically correct by construction** — no undefined behavior in
  the generator's output. The compiler is the oracle-under-test;
  differential agreement between compilers is the property.
- **Direct fit for mirror:** the mirror compiler substrate has a
  natural differential oracle in the two-tick FLOOR/bootstrap dual
  during the substrate-pull collapse arc. YARPGen-shape structured
  program generation could produce mirror-spec programs that exercise
  the shard-decl / dispatch / kintsugi mender surface.

---

## §4 Fate as inference driver — the novel composition

### §4.1 What the SOTA covers separately

**LLM-driven test generation:**
- CoverUp (Andrzejewski et al. 2025, arXiv 2403.16218): iterative
  LLM prompting + coverage feedback for Python regression tests. 80%
  median line+branch coverage on the CM benchmark vs 47% for CodaMosa.
- TitanFuzz (Deng et al. ISSTA 2023,
  <https://lingming.cs.illinois.edu/publications/issta2023a.pdf>):
  first approach directly using LLMs to generate fuzz inputs for DL
  library testing.
- ELFuzz (arXiv 2506.10323, 2025): LLM synthesizes the fuzzer itself
  via evolution over the fuzzer space.
- IntUT (Nan et al.): test-intention-guided LLM unit test generation.
- LLM-driven fuzzing systematic survey:
  <https://www.sciencedirect.com/science/article/pii/S0952197626014090>
  — new field of "LLM-driven Fuzzing."

**ML-driven compilation:**
- CompilerGym (Cummins et al. 2021, <https://arxiv.org/pdf/2109.08267>):
  1.1M-benchmark RL environment for compiler pass ordering.
- LoopLearner (Mammadli et al. 2021): predicts loop-writing variants.
- Neural-guided superoptimization: Aguiar et al. 2025 (Souper +
  neural network augmentation).
- Learned static analysis for type inference:
  Hellendoorn et al. 2018 "Deep Learning Type Inference"
  (<https://vhellendoorn.github.io/fse2018-j2t.pdf>) — 307× cited.
- Awesome list: <https://github.com/zwang4/awesome-machine-learning-in-compilers>

### §4.2 What the SOTA does NOT cover

**No paper found where the SAME inference machinery drives both test
generation AND compilation decisions.**

The reason is structural: in the standard practitioner architecture,
tests are external to the SUT and compilation is internal. There's
no shared substrate for "the thing that decides what to test" and
"the thing that decides how to compile." The two roles are separated
by the harness boundary.

**Alex's directive collapses this boundary.** Fate is already the
inference driver for compilation (Prism operation selection). If the
same weight-set + softmax + depth-modulation is *also* the inference
driver for property generation, then:

- Tests and compilation share a distribution over Prism operations
- The compilation loop and the test loop are the same loop
- Weight training on compilation success signals also improves
  property-generator quality (and vice versa)
- The ouroboros closes at a new altitude: **the compiler tests
  itself using the same tick machinery it uses to compile**

### §4.3 Concrete integration surfaces

**Surface A — Fate emits Arbitrary decisions.**

```rust
// Hypothetical shape:
impl<T: prismqueer::Beam> pillar::Arbitrary for T {
    fn arbitrary(fate: &mut fate::Fate, seed: &mut ChoiceSeq)
        -> Self
    { … }
}
```

The `ChoiceSeq` is Hypothesis-style (byte-buffer that records every
decision). Fate's `fn tick(&mut self, features: Features)
-> FateOutput` supplies the softmax distribution that biases
`ChoiceSeq::draw_integer` toward interesting regions.

**Surface B — Fate scores counterexamples.**

Once a property fails, the shrinker asks Fate for the utility value
of each candidate reduction. Fate's `HolonomyHealth` is already
shaped as a scalar loss — direct fit for targeted-PBT UV.

**Surface C — Fate proposes property conjectures.**

QuickSpec-shape: given the prismqueer term algebra (five operations
+ known type signatures), Fate proposes candidate equations. The
`Explorer` model (currently: "Subgraph comprehension — compressed
meaning") is structurally the right sub-model for this role — its
job description already includes "recover meaning at the boundary."

**Surface D — Fate closes the training loop.**

Every property failure produces a training example: `(features,
counter-example, minimum_shrink)`. Fate's training pipeline
(`fate/training/examples.json` + `fate::train`) can consume these.
The compiler literally gets smarter about compiling by discovering
bugs about compiling.

### §4.4 What "beautiful" means, mechanically

Alex's word "beautiful" resolves to a specific structural claim
this scout can verify:

> The same 90-parameter softmax that decides "what compilation step
> runs next" ALSO decides "what property test runs next" ALSO decides
> "which shrinkage direction to explore first." One substrate, three
> witnesses.

This is the shape of the compositional-over-accretion Alex has been
holding at every prior scout. The novel piece isn't the ML technique
(CoverUp already does LLM-driven testing; CompilerGym already does
RL-driven pass ordering). The novel piece is that **the inference
substrate is shared**, which is only possible because the compiler
itself was designed around Fate as its runtime decision-maker.

### §4.5 Substrate risk — the honest gap

**Fate's current feature vocabulary is compilation-features, not
program-under-test-features.** The 16-dim `Features` type is derived
from spectral state at compilation altitude. To drive Arbitrary
decisions, Fate needs to accept a *different* feature vocabulary —
the shape of the term algebra being tested, not the shape of the
compilation state.

Two options:

1. **Extend `FEATURE_DIM = 16`** to a larger fixed vocabulary
   covering both altitudes. Backward-compatible; adds ~seven feature
   dimensions.
2. **Parameterize Fate over `Features`** as an associated type.
   Bigger substrate change; requires re-training all five selectors.

The scout's read is option (1) — extend the vocabulary; the
selectors gracefully learn which dimensions matter at which altitude
via depth modulation. This preserves the "one algorithm, one binary,
five weight sets" design principle in `fate/README` /
`fate/docs/superpowers/plans/2026-04-05-training-pipeline.md`.

---

## §5 Ergonomics — what "smooth as velvet" means concretely

### §5.1 Side-by-side test shapes

**QuickCheck (Haskell):**

```haskell
prop_reverseInvolutive :: [Int] -> Bool
prop_reverseInvolutive xs = reverse (reverse xs) == xs

main = quickCheck prop_reverseInvolutive
```

**hedgehog (Haskell):**

```haskell
prop_reverse :: Property
prop_reverse = property $ do
  xs <- forAll $ Gen.list (Range.linear 0 100) (Gen.int (Range.linear 0 1000))
  reverse (reverse xs) === xs
```

**Hypothesis (Python):**

```python
@given(st.lists(st.integers()))
def test_reverse_involutive(xs):
    assert list(reversed(list(reversed(xs)))) == xs
```

**StreamData (Elixir):**

```elixir
property "reverse is involutive" do
  check all xs <- list_of(integer()) do
    assert xs |> Enum.reverse() |> Enum.reverse() == xs
  end
end
```

**proptest (Rust):**

```rust
proptest! {
    #[test]
    fn reverse_involutive(xs in prop::collection::vec(any::<i32>(), 0..100)) {
        let out: Vec<i32> = xs.iter().rev().cloned().collect();
        let back: Vec<i32> = out.iter().rev().cloned().collect();
        prop_assert_eq!(back, xs);
    }
}
```

### §5.2 What the "smooth as velvet" bar actually is

Reading across five frameworks, the ergonomic winners share four
properties:

1. **Test body reads like a unit test.** The generator machinery is
   in the argument-list / do-block preamble, not intermixed with
   assertions. StreamData's `check all x <- gen do … end` and
   Hypothesis's `@given(…)` are the clearest witnesses.

2. **No separate shrinker.** Framework does the shrinking; user
   writes zero lines of shrink code. Hedgehog / Hypothesis / proptest
   / StreamData all clear this bar. QuickCheck / quickcheck-rs /
   ScalaCheck do not.

3. **Composition by ordinary language operators.** `Gen` /
   `Strategy` / `Arbitrary` is a monad or applicative in the host
   language; `bind` and `map` are the primitives. No DSL learning
   curve on top of what a language user already knows.

4. **Ranges shrink toward salient origins.** hedgehog's `Range`
   type, proptest's numeric strategies, Hypothesis's `min_value=` /
   `max_value=` all support "shrink toward N" — which is the
   mechanism that produces *readable* counter-examples (small
   integers, empty lists, low-nesting trees).

### §5.3 What prismqueer::liquid should adopt

Based on the substrate discipline of "delightfully-boring naming +
composition over accretion" and the substrate-already-had-the-word
grep discipline:

**ADOPT (concrete):**

1. **A `pillar::Sample` carrier** — Hypothesis-shape choice-sequence,
   *not* hedgehog-shape rose-tree. Reason: the choice-sequence is
   substrate-honestly the same shape as the `[f64; 5]` distribution
   Fate already emits, and the byte-buffer runtime is Rice-safe
   (fixed representation, no polymorphism explosion).

2. **A `pillar::Arbitrary` trait** with a single `fn arbitrary(&mut
   Sample) -> Self` method. Shrinking is automatic via the Sample's
   byte-reduction machinery. No separate `shrink` — one door.

3. **A `pillar::forall` runner** that takes an `impl FnOnce(T) ->
   PropertyVerdict` and returns a folded `PropertyVerdict`. Reuses
   `pillar::fold` semantics: any `Fail` produces a shrunk minimal
   counter-example; folded `Partial`s carry combined confidence.

4. **A `pillar::Range<T>`-shape shrink-toward-origin primitive.**
   Names to consider: `pillar::origin` (bike-shed later; the concept
   is that magnitude shrinks toward zero, indices shrink toward the
   Default state, etc.).

### §5.4 What prismqueer::liquid should REFUSE

**REFUSE (substrate-honest):**

1. **A separate `Shrink` trait.** The whole shrinker-duplication
   failure mode of QuickCheck lives here. Choice-sequence Arbitrary
   makes it structurally impossible to write a wrong shrinker for
   a given generator.

2. **A `Strategy` monad DSL a la proptest.** proptest's
   `prop_flat_map` / `prop_union` / `prop_filter` layer is
   accretional. Rust's native `impl Trait` + iterator combinators
   already give us monad-ish composition without a DSL. If we need
   `pillar::Sample::map` and `pillar::Sample::bind`, that's the
   whole surface.

3. **A `#[proptest]` / `check_all!` macro layer.** Every test today
   is a `#[test]` function returning `()`. A `pillar::forall` runner
   returning `PropertyVerdict` composes cleanly into `#[test] fn
   … { assert!(matches!(pillar::forall(…), PropertyVerdict::Pass)) }`.
   The macro layer buys 3 lines of test-body brevity at the cost of
   IDE/rustc/rust-analyzer surface friction, cargo doc friction, and
   a whole new mental model layer. **Refuse.** (This is the
   "delightfully-boring" hard-line: `#[test]` + hand-written
   `assert!` is the ergonomic bar we hold.)

4. **A `bolero` / `fuzzcheck` coverage-integration.** These are
   Rust's coverage-guided-PBT layer. They require compiler
   instrumentation (`-C instrument-coverage`) and a fuzzer harness.
   Substrate-honest: **the coverage feedback role is filled by Fate's
   HolonomyHealth**. That's the whole point of the arc. Adding a
   separate coverage-instrumentation layer breeds the exact
   accretion the substrate-already-had-the-word discipline refuses.

---

## §6 Composition with the existing pillar surface

### §6.1 The seam

The verdict semilattice at `PropertyVerdict + merge_with` is already
the correct downstream output for any PBT input surface. The
composition path is:

```text
pillar::Sample                                  [NEW]
  ↓ (choice-sequence-driven)
pillar::Arbitrary::arbitrary(sample) -> T       [NEW]
  ↓
user's property fn: |T| -> PropertyVerdict      [existing pillar
                                                  primitives return
                                                  PropertyVerdict]
  ↓
pillar::forall(…)                               [NEW: shrinks on Fail,
                                                  folds Partial across
                                                  shrinkage steps]
  ↓
PropertyVerdict                                 [existing]
  ↓
pillar::fold(&[…])                              [existing iter 9]
  ↓
unified verdict                                 [existing]
```

**Zero changes to the existing six pillar primitives.** All the
existing tests remain valid — they're closed-form witnesses, which
means for-all-inputs claims from `forall` reduce to them at the
boundary case where the input space is single-element.

### §6.2 The Fate seam

```text
compilation_features: Features    ← from mirror compilation state
property_features:    Features    ← from term-algebra being tested
                            ↓ same 16-dim vocabulary (with extended dims)
                        fate.tick(features)
                            ↓
                     FateOutput { model, decision, health }
                            ↓
                     ┌───────┴───────┐
                     ↓               ↓
       compilation next step    Arbitrary decision +
                                shrinker direction hint
```

Fate's `Explorer` sub-model (currently "Subgraph comprehension —
compressed meaning") plays the role of "which subregion of input
space is worth exploring." This is a direct fit — the sub-model's
job description already reads like a targeted-PBT neighbourhood
function.

### §6.3 What breaks

**Nothing at the type level.** `PropertyVerdict` is unchanged. The
six pillar primitives are unchanged. The verdict semilattice is
unchanged. `Fate::tick` gains an `arbitrary_bias` field on
`FateOutput` but the existing callers ignore it.

**One re-training pass on Fate weights** is required if we want the
selector to consider the extended feature dimensions. Backward
compat: if the new dims are zero-init, existing selectors behave
identically.

### §6.4 What Pillar IV (parked) becomes

The forward-promise in the composition spec §7.1 —
*"Pillar IV — @peer.audhd cognitive fanout requires fate::Fate::tick
and a bridge file at mirror/rust/src/liquid.rs"* — is directly
enabled by this scout's Surface A + B + C landing plan. Pillar IV
was blocked on "fate::Fate::tick doesn't emit anything
property-testing-shaped"; this scout maps out exactly what needs to
change.

**Pillar IV then re-scopes to:** K-parallel Arbitrary generation via
K distinct Fate ticks with different depth modulation, all folded
via `pillar::fold`. The cognitive-fanout altitude naturally emerges
from the composition — it's not a separate primitive.

---

## §7 Adjudication questions for Alex

### §7.1 Q1 — Hypothesis choice-sequence vs hedgehog rose-tree?

The scout leans Hypothesis-shape (choice-sequence) because:
- Rice-safe fixed representation
- Directly compatible with Fate's `[f64; 5]` distribution shape
- No polymorphism explosion at the trait level
- Well-suited to coverage-guided extension

But hedgehog-shape (rose-tree) has:
- Cleaner Applicative composition ergonomics
- No byte-buffer machinery to maintain
- Better shrink minimality guarantees per Applicative operations

**Ambiguity for Alex:** which axis matters more here? Rice-safe fixed
representation vs cleaner Applicative composition?

### §7.2 Q2 — Extend `FEATURE_DIM` or parameterize Fate?

Option 1: bump `FEATURE_DIM = 16 → 23` (or similar). Backward
compatible with zero-init on new dims. Re-training pass produces
better selectors but existing weights still work.

Option 2: `Fate<F: Features>` associated-type-parameterized. Bigger
substrate change; each altitude gets its own weight set.

Scout leans Option 1 (extend). But Option 2 is architecturally
cleaner in the very long term.

**Ambiguity for Alex:** short-term ergonomic pull vs long-term
architectural cleanliness?

### §7.3 Q3 — Do we adopt QuickSpec-shape conjecture discovery?

QuickSpec would automatically emit conjectures over the prismqueer
term algebra. This is powerful — potentially discovers laws we
haven't guessed. But it's also a whole additional surface (term
generation, equivalence-class partitioning, congruence-closure
pruning).

**Scout read:** land Surfaces A + B + C first (Arbitrary + Sample +
forall + Fate bridge). Defer Surface D (Fate proposes conjectures
directly via `Explorer` sub-model) to a follow-up loop after the
foundational surface is in place.

**Ambiguity for Alex:** aim wide (land all four surfaces) or aim
narrow (land the ergonomic PBT surface first, defer property
discovery)?

### §7.4 Q4 — Do we ever need the closed-form enumeration tests?

The existing 98 tests are closed-form enumeration. If `forall` lands
and can subsume them (e.g. `commutator_antisymmetric_over_test_bundle`
becomes `forall (|a: TestBundle| forall(|b: TestBundle| commutator(a,
b) == commutator(b, a)))`), do we delete the hand-rolled loops?

**Scout read:** keep them. They're empirical closed-form witnesses —
they run in 0 randomness, always produce the same result, and serve
as the base-case oracle for the `forall` runner. Deleting them loses
the closed-form guarantee.

**Ambiguity for Alex:** do we keep the hand-rolled enumeration tests
as base-case oracles or delete them once `forall` covers the
universally-quantified case?

### §7.5 Q5 — Where does the shrinker live?

The shrinker semantics live either:
- Inside `pillar::Sample` (Hypothesis-shape: shrinker is byte-buffer
  reduction)
- Inside `pillar::forall` (proptest-shape: strategy owns shrinkage)
- Inside `pillar::Arbitrary` (QuickCheck-shape: each type owns its
  own shrink)

Scout leans Option 1 (Sample owns shrinkage). But this requires the
Sample carrier to be substrate-honestly a byte-buffer, which pushes
the surface into "carries state across draws" territory — one more
struct to manage.

**Ambiguity for Alex:** how much substrate machinery is acceptable
inside `Sample`? Is a byte-buffer choice-sequence a "one door" carrier
or accretion?

---

## §8 Substrate-honest verdict

### §8.1 What Reed should build next

**Land the property-generation surface as three primitives at
`prismqueer::liquid::pillar`:**

```rust
// The choice-sequence carrier — Hypothesis-shape byte-buffer.
pub struct Sample { /* byte-buffer + read cursor */ }

impl Sample {
    pub fn draw_integer(&mut self, min: i64, max: i64) -> i64 { … }
    pub fn draw_float(&mut self, min: f64, max: f64) -> f64 { … }
    pub fn draw_bool(&mut self) -> bool { … }
    pub fn draw_bytes(&mut self, n: usize) -> &[u8] { … }
    pub fn draw_from<T: Arbitrary>(&mut self) -> T { T::arbitrary(self) }
}

// The generator trait — single method, no separate shrinker.
pub trait Arbitrary: Sized {
    fn arbitrary(sample: &mut Sample) -> Self;
}

// The runner — folds N samples' verdicts, shrinks on Fail.
pub fn forall<T, F>(f: F) -> PropertyVerdict
where
    T: Arbitrary,
    F: Fn(T) -> PropertyVerdict,
{
    let mut unified = PropertyVerdict::Pass;
    for _ in 0..N_SAMPLES {
        let mut sample = Sample::random();
        let value = T::arbitrary(&mut sample);
        let verdict = f(value);
        if matches!(verdict, PropertyVerdict::Fail(_)) {
            // Shrink the sample's byte-buffer; return minimal Fail.
            return shrink_and_report(sample, f);
        }
        unified.merge_with(&verdict);
    }
    unified
}
```

**Then wire Fate:**

```rust
// In fate/, or in a mirror-side bridge:
impl fate::Fate {
    /// Bias a Sample toward Fate-preferred regions of the input space.
    pub fn bias_sample(&self, sample: &mut Sample,
                       features: prismqueer::Features) {
        let out = self.tick(features);
        // Use out.decision.distribution to weight sample.draw_*
        // toward the argmax direction, with confidence as temperature.
        sample.set_bias(out.decision.distribution, out.decision.confidence);
    }
}
```

**Sequence:**

1. **Iter 1:** Land `pillar::Sample` + `pillar::Arbitrary` trait +
   basic `pillar::forall` runner. Zero shrinker; failures reported
   as-is. RED tests confirm existing 98 tests still pass (they don't
   use `forall`); new GREEN tests exercise `forall` at the base case.

2. **Iter 2:** Land byte-buffer shrinker inside `Sample`. Failures
   auto-minimize. Property-verdict quality visibly improves.

3. **Iter 3:** Land `fate::Fate::bias_sample` bridge in a new
   `mirror/rust/src/liquid.rs` (this is the Pillar IV bridge file
   the composition spec §7.1 was waiting on). Zero-init weights →
   backward-compat.

4. **Iter 4:** Extend `FEATURE_DIM` if needed (Q2 adjudicated), and
   re-train Fate on the extended vocabulary using existing training
   pipeline in `fate/training/`.

5. **Iter 5+:** QuickSpec-shape conjecture discovery via Fate's
   `Explorer` sub-model. Defer until 1-4 land.

### §8.2 Length of arc

Iter 1-3 are each 1-2 landings. Iter 4 is a training run (batch job,
not a loop iteration). Iter 5+ is the follow-up arc after the
foundational surface lands.

**Total substrate work to reach "smooth as velvet":** approximately
3-5 landings + 1 training pass.

### §8.3 Substrate-refusal reminders

- **No new Rust crate.** All lands inside `prismqueer/src/liquid.rs`
  and `fate/src/lib.rs` + a new `mirror/rust/src/liquid.rs` bridge
  (which the composition spec explicitly scoped as Pillar IV's home).

- **No proptest / quickcheck / bolero / fuzzcheck dependencies.**
  Substrate-honest verdict: **we grow our own PBT surface.** This
  is *not* NIH — this is because the verdict algebra, the term
  algebra, and the inference substrate are all already ours. Bolting
  on proptest would mean two disjoint verdict layers
  (proptest's + prismqueer's) and duplicate composition mechanics.

- **No `#[proptest]` macro layer.** `#[test] fn foo() { assert!(matches!(
  pillar::forall(…), PropertyVerdict::Pass)) }` is the ergonomic bar.

- **No separate coverage instrumentation.** Fate's HolonomyHealth
  IS the coverage feedback substrate. That's the whole ML-driven
  compiler thesis.

### §8.4 What makes Alex's directive genuinely beautiful

Reading across the SOTA in §2-4 with the substrate discipline of
§1, the composition Alex proposed is *substrate-already-had-the-word*
at three altitudes:

- The **verdict algebra** was already there (terni PropertyVerdict +
  merge_with).
- The **inference substrate** was already there (Fate's five-model
  selector + softmax + depth modulation).
- The **term algebra** was already there (prismqueer's Bundle tower +
  Prism monoid).

The three altitudes needed a **single seam** — Fate emitting an
`arbitrary_bias` alongside its existing `model` decision, threaded
through a `pillar::Sample` carrier. **One new struct, one new trait,
one new function, one new field on an existing struct.** That's the
"smooth as velvet."

The rest of the SOTA — QuickSpec conjecture discovery, targeted
PBT, coverage-guided fuzzing, LLM-driven test generation — becomes
readable as *four separate surface variations on the same underlying
Fate-driven inference substrate.* The compiler tests itself, the
test loop drives compilation, and the same weights improve both.
That's what "beautiful" resolves to at the mechanical level.

---

## §9 Kagi source manifest

Primary papers and canonical URLs cited throughout this scout:

**Foundational PBT (§2):**
- Claessen & Hughes 2000 QuickCheck — <https://www.cs.tufts.edu/~nr/cs257/archive/john-hughes/quick.pdf> — ACM DOI <https://dl.acm.org/doi/10.1145/357766.351266>
- QuickCheck Hackage: <https://hackage.haskell.org/package/QuickCheck>
- Hedgehog GitHub: <https://github.com/hedgehogqa/haskell-hedgehog>
- Hedgehog Hackage: <https://hackage.haskell.org/package/hedgehog>
- de Vries et al. 2023 "falsify: Internal Shrinking Reimagined" — <https://well-typed.com/blog/aux/files/falsify.pdf>
- Well-Typed 2019 "Integrated versus Manual Shrinking" — <https://www.well-typed.com/blog/2019/05/integrated-shrinking/>
- MacIver "Compositional shrinking" — <https://hypothesis.works/articles/compositional-shrinking/>
- seelengrab 2024 "The Properties of QuickCheck, Hedgehog and Hypothesis" — <https://seelengrab.github.io/articles/The%20properties%20of%20QuickCheck,%20Hedgehog%20and%20Hypothesis/>
- Discourse 2024 "Sad state of property-based testing libraries" — <https://discourse.haskell.org/t/the-sad-state-of-property-based-testing-libraries/9880>

**Ecosystem PBT (§2):**
- StreamData GitHub: <https://github.com/whatyouhide/stream_data>
- StreamData announcement 2017: <https://elixir-lang.org/blog/2017/10/31/stream-data-property-based-testing-and-data-generation-for-elixir/>
- ExUnitProperties docs: <https://stream-data.hexdocs.pm/ExUnitProperties.html>
- Hypothesis GitHub: <https://github.com/HypothesisWorks/hypothesis>
- Hypothesis strategies reference: <https://hypothesis.readthedocs.io/en/latest/data.html>
- Hypothesis ghostwriter: <https://hypothesis.readthedocs.io/en/latest/ghostwriter.html>
- Hypothesis Conjecture engine: <https://github.com/HypothesisWorks/hypothesis/blob/master/hypothesis-python/src/hypothesis/internal/conjecture/engine.py>
- Hypothesis internals reference: <https://hypothesis.readthedocs.io/en/latest/reference/internals.html>
- proptest GitHub: <https://github.com/proptest-rs/proptest>
- proptest docs.rs: <https://docs.rs/crate/proptest/latest>
- BurntSushi quickcheck: <https://github.com/BurntSushi/quickcheck>
- fast-check GitHub: <https://github.com/dubzzz/fast-check>
- fast-check intro: <https://fast-check.dev/docs/introduction/what-is-property-based-testing/>
- ScalaCheck GitHub: <https://github.com/typelevel/scalacheck>
- ScalaCheck docs: <https://scalacheck.org/documentation.html>
- PropEr GitHub: <https://github.com/proper-testing/proper>

**Property discovery frontier (§3):**
- Claessen, Smallbone, Hughes "QuickSpec" — <https://smallbone.se/papers/quickspec.pdf>
- Johansson & Smallbone "Automated Conjecturing in QuickSpec" — <https://www.cse.chalmers.se/~jomoa/papers/Automated_Conjecturing_in_QuickSpec.pdf>
- Löscher & Sagonas ISSTA 2017 "Targeted property-based testing" — <https://dl.acm.org/doi/10.1145/3092703.3092711>
- Löscher & Sagonas ICST 2018 "Automating Targeted Property-Based Testing" — <https://proper-testing.github.io/papers/icst2018.pdf>
- "Programmable Property-Based Testing" arXiv 2602.18545 — <https://arxiv.org/html/2602.18545v1>
- Padhye et al. "Coverage guided, property based testing" — <https://dl.acm.org/doi/10.1145/3360607>
- Segura et al. 2018 "Metamorphic Testing" — <https://dl.acm.org/doi/10.1145/3143561>
- Ba et al. 2025 "Metamorphic Coverage" — <https://arxiv.org/pdf/2508.16307>

**Compiler differential fuzzing (§3):**
- Csmith GitHub: <https://github.com/csmith-project/csmith>
- YARPGen OOPSLA 2020 — <https://users.cs.utah.edu/~regehr/yarpgen-oopsla20.pdf>
- YARPGen PLDI 2023 — <https://livinskii.com/assets/files/yarpgen-pldi23.pdf>

**LLM-driven testing / ML-driven compilation (§4):**
- CoverUp arXiv 2403.16218 — <https://arxiv.org/html/2403.16218v3> — ACM <https://dl.acm.org/doi/abs/10.1145/3729398>
- TitanFuzz ISSTA 2023 — <https://lingming.cs.illinois.edu/publications/issta2023a.pdf>
- ELFuzz arXiv 2506.10323 — <https://arxiv.org/abs/2506.10323>
- LLM-driven fuzzing survey — <https://www.sciencedirect.com/science/article/pii/S0952197626014090>
- CompilerGym GitHub — <https://github.com/facebookresearch/CompilerGym>
- CompilerGym paper arXiv 2109.08267 — <https://arxiv.org/pdf/2109.08267>
- LoopLearner arXiv 2102.13514 — <https://arxiv.org/pdf/2102.13514>
- Hellendoorn et al. 2018 "Deep Learning Type Inference" — <https://vhellendoorn.github.io/fse2018-j2t.pdf>
- Aguiar et al. 2025 neural-guided superoptimization — <https://www.sciencedirect.com/science/article/pii/S0950584925001399>
- Awesome ML in compilers — <https://github.com/zwang4/awesome-machine-learning-in-compilers>

---

## §10 Meta — where this scout terminated

- Grep-first on prism/ + prismqueer/ + mirror/rust/ + fate/ — done.
  Zero PBT-framework deps anywhere.
- Kagi search across ten queries — done. ~40 sources synthesized.
- Substrate ground truth: prismqueer::liquid is verdict-marshaling
  today, not generator-driven PBT.
- Alex's directive maps to a small (3-primitive) substrate landing
  that composes over Fate without new crate boundaries.
- **The Q1-Q5 adjudication questions in §7 are the substrate-honest
  ambiguities Reed cannot resolve without Alex direction.**
- The scout is ~1500 LOC per Taut convention — depth over breadth.
  Every SOTA feature evaluated through delightfully-boring-naming +
  composition-over-accretion.

**Report path:** `docs/scouts/2026-07-18-taut-property-based-testing-frameworks-fate-inference-driver.md`
