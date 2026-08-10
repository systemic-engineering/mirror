# Beta-Normal AST Content-Addressing — Canonical Spec

**Author:** Mara `<mara@systemic.engineer>`. 2026-08-10.
**Register:** Mara-substrate canonical spec. Composition-not-taxonomy. Delightfully-boring precision. Substrate-decl throughout. Substrate-honest is the mode.
**Composes over:**

- **Alex 2026-08-10 verbatim naming** (§0.1 below) — beta-normalization + Dhall prior art directive.
- **Taut scout `ab3821e`** at `docs/scouts/2026-08-10-taut-prism-block-compiler-consumption-verification.md` — empirical verdict that crystal-OID is byte-verbatim at both altitudes; `resugar ∘ sugar = id` currently aspirational, not empirical.
- **Reed Fire E landings**: M-E1 (`acaed91`; 6 shard mints — 4 fracture species + `@magic/reveal/expand` + `@kintsugi/mend/sugar`) + M-E2 (`0021882`; `apply_h::act` shard-body-projector extension with P1 identity-carrier detector at `rust/src/apply_h.rs:246-336`; 23/23 tests GREEN).
- **Mara 2026-08-09 kintsugi-sugar canonical spec** at `docs/specs/2026-08-09-mara-kintsugi-sugar-desugar-composition-canonical-spec.md` — the sugar-desugar-resugar composition this spec makes empirically load-bearing.
- **Mara 2026-08-09 A_F universality math** at `docs/math/2026-08-09-mara-a-f-universality-kintsugi-sugar-mechanism.md` — the A_F universal-structure claim that motivates why identity projections are elidable at every shard.
- **Mara 2026-08-09 physics insight** at `/Users/reed/dev/systemic.engineering/practice/insights/spectral/mass-discrete-spacetime-continuous-through-5op-spectral-space.md` §1.1 Theorem 1.1 — A_F IS the prismqueer 5-op void-duality algebra.
- **Reed 2026-03-01 Dhall research** at `/Users/reed/dev/systemic.engineering/practice/insights/infrastructure/semantic-hashing-normalization.md` — the corpus's earliest recognition of Dhall's beta-normalization + semantic hashing pipeline.
- **Alex+Reed 2026-03-28 recognition** at `/Users/reed/dev/systemic.engineering/practice/insights/coincidence/kolmogorov-canonical-complexity.md` — sub-Turing + beta-normalization + content-addressing chain assembled explicitly in the corpus.

**Cascade context:** Fire E revision. Reed's Fire E M-E3/M-E3.5 was proposed as a *workaround* (read-path projection primitive + bootstrap namespace resolver fallback) for the empirical fact that removing prism-block source bytes changes crystal-OID at both the source-byte cache altitude (Primitive A) AND the AST altitude (Primitive B) — a substrate-decidable failure of `resugar ∘ sugar = id`. This spec names the **issue-under-the-issue** Alex probed for: the compiler's AST over-represents the substrate; it hashes source-text tokens verbatim including patterns (A_F identity projection at every shard) that are structurally universal. The substrate-honest fix is not a read-path projection — it is **beta-normalization at the AST altitude before hashing**, per Dhall's landed prior art. Under this substrate, `resugar ∘ sugar = id` holds by Church-Rosser confluence, not by careful byte-level round-tripping. M-E3/M-E3.5 workaround obsoletes; walker cascade M-E4 runs on top of beta-normal-AST hashing at a Fire D-adjacent tick.

---

## §0 Substrate-honest pre-position

### §0.1 Alex 2026-08-10 verbatim (load-bearing)

> "Which role does beta-normalization of the AST play here? [...] Dhall does this. They have semantic hashing where the hash is based on the beta-normalized AST, which is sexy AF, they use it for safe remote imports. I'm pretty sure there's already something on that in the corpus at `~/dev/systemic.engineering/practice/insights/` and/or in docs `@../spectral/docs`"

Two load-bearing entailments:

1. **Beta-normalization at AST altitude is the substrate-honest fix.** Not read-path projection. Not bootstrap fallback. Hash the beta-normal form of the AST; source variation that beta-reduces to the same normal form hashes identically.
2. **Substrate-already-had-the-word.** The corpus explicitly names Dhall's semantic hashing pipeline in three separate insight files (§3.4 grep verification below). This is not a novel invention; this is naming what the substrate had been circling since 2026-03-01 and had partially cited in the 2026-06-05 spectral MCP surface spec.

### §0.2 The issue-under-the-issue (Alex 2026-08-10 probe)

Reed halted Fire E at M-E3 asking whether prism blocks are load-bearing at compile/parse/crystal-canonical altitude, or documentation-only. Taut's empirical scout (`ab3821e`) verified: removing prism block source bytes changes crystal-OID via source-byte cache (Primitive A: `canonical_hash` at `bootstrap/src/lib.rs:679,683,794,803`) AND via AST content-address (Primitive B: `compute_content_oid` at `bootstrap/src/spectral.rs:162-181` walking AST via `Fold5` reducer). Reed proposed workaround: M-E3 read-path projection primitive + M-E3.5 bootstrap namespace resolver fallback.

Alex probed: *"What's the issue under the issue?"*

The issue under the issue is: **the compiler's AST over-represents the substrate.** It hashes source-text tokens verbatim, including patterns that are structurally universal (A_F identity projection at every shard, per Mara 2026-08-09 physics insight §1.1: A_F is universal across shards; only H_shard varies). Two shards with the same substrate content — one sugar-omitted, one full-form — produce different AST-OIDs because the AST is a *syntactic* projection, not a *semantic* one.

The workaround (M-E3 read-path projection) treats the symptom: reconstruct the omitted bytes on read so downstream hashers see identical bytes. The substrate-honest fix treats the cause: **canonicalize the AST at hash time so semantically-equivalent variations map to the same OID by construction, without byte-level reconstruction.**

### §0.3 Two paths refused; substrate-honest is the mode

There is no "here's the honest approach / here's the fast approach; I recommend fast." There is one substrate-honest composition: beta-normalize the AST before hashing, per Dhall's landed spec. Per feedback `feedback-substrate-honest-is-the-mode` (Alex 2026-07-07). Two-paths framing is refused. The workaround already surfaced by Reed (M-E3 read-path projection) is not a "fast path" alternative to this spec — it is the wrong altitude of intervention that this spec obsoletes.

### §0.4 Bootstrap-is-dead preservation

Per Alex 2026-07-22 (memory `bootstrap_is_dead_do_not_propose_bootstrap_altitude_solutions`): bootstrap/ is the @roomba+@kintsugi collapse target; do not propose bootstrap-altitude solutions. This spec lands the beta-normalizer at **rust/ altitude** — as a shard-body composing over `@io/fs.read` + apply_h::act + Fold5 primitives at a Fire D M5-adjacent tick. Not at bootstrap altitude. bootstrap/ inherits the substrate change through its retirement pathway; it is not the authoring surface.

### §0.5 Karen anti-theft: ancestor-at-introduction-site

Every claim below carries its ancestor named at first mention. Recognition ancestry: Recognition #79 (5-op = A_F projector basis; Mara + Reed 2026-06-18); Mara 2026-08-09 physics insight §1.1 (A_F universality); Mara 2026-08-09 kintsugi-sugar spec §5 (A_F universality justification at compiler altitude). External corpus: Dhall language standard (`dhall-lang/dhall-lang` repo `standard/*.md`); Chamseddine-Connes-Marcolli 2007 (arXiv:hep-th/0610241) via inheritance from Mara insight; Church 1936 (β-reduction); Church-Rosser 1936 (confluence); de Bruijn 1972 (index-shift substitution); Barendregt 1984 (*The Lambda Calculus: Its Syntax and Semantics*); Unison / Frank McSherry lineage (content-addressed code with AST-hashing). Landed substrate: Reed 2026-03-01 Dhall research + Reed+Alex 2026-03-28 sub-Turing + normalization chain + spectral 2026-06-05 MCP surface spec §6.2 (Dhall as design-influence).

---

## §1 The issue-under-the-issue — compiler AST over-represents substrate

### §1.1 What the current compiler does

Two altitudes of "crystal OID" in bootstrap (Taut §3.1):

- **Primitive A — `bootstrap/src/hash.rs::canonical_hash`** (CoincidenceHash<5,5>): byte-verbatim hash of raw source. Any whitespace change, comment change, or prism-block removal changes `source_oid`.
- **Primitive B — `bootstrap/src/spectral.rs::compute_content_oid(&ast)`** (Fold5 Dirac action over AST): walks the AST via `Fold5` reducer dispatching on `AstKind`. The reducer emits kind-tagged hash bytes per node; `Dark` nodes hash verbatim under the `"dark"` tag. Whitespace between tokens does NOT change the AST, but structurally-universal patterns (like the P1 identity-carrier prism block) DO change the AST — different AST tree → different `compute_content_oid` result.

Taut §3.2 verdict, verbatim: *"Crystallization at the current bootstrap altitude is NOT byte-canonical with respect to prism-block presence. Removing a prism block CHANGES the crystal OID at both the source-cache altitude (Primitive A) and the AST altitude (Primitive B)."*

### §1.2 Why this is the wrong altitude

The AST hasn't been asked to distinguish "different substrate content" from "different source-text expressions of the same substrate content." It hashes what it parses; what it parses is source-text tokens; source-text tokens carry both substrate content and syntactic sugar.

For P1 identity-carrier prism blocks (Fire E M-E2 landed at Reed `0021882`; detector at `rust/src/apply_h.rs:246-336`), the block `prism @X { focus X / project X / split X / shift X / settle X }` is the **identity projection of A_F over H_shard=self** (Mara 2026-08-09 physics insight §1.1 + kintsugi-sugar spec §5). Every shard restates the same A_F; only the carrier slot differs. This is A_F acting as identity on the shard's own carrier. The identity element of an algebra reduces away under normalization; keeping it in the AST is a syntactic accident, not a semantic requirement.

### §1.3 The substrate-honest fix, in one line

**Hash the beta-normal form of the AST, not the AST verbatim.**

Under this substrate: identity projections beta-reduce away; the beta-normal AST does not carry them; two shards (one sugar-omitted, one full-form) beta-normalize to the same AST and thus hash to the same OID. `resugar ∘ sugar = id` at OID altitude holds by Church-Rosser confluence, not by careful byte-level round-tripping.

Reed's M-E3 read-path projection (proposed at `rust/src/apply_h.rs::project_p1_identity_prism_at`) is unnecessary under this substrate: there is no need to reconstruct bytes at read time because the hasher never saw the bytes in the first place — it saw the beta-normal AST, which is identical whether or not the source carries the identity-projection sugar.

---

## §2 Beta-normalization as the substrate-honest fix

### §2.1 Where beta-normalization sits in the pipeline

Current pipeline (Taut §3.1):

```
source_bytes  ─┬─→  canonical_hash        ─→  source_oid       (Primitive A)
               │
               └─→  parse  ─→  AST  ─→  compute_content_oid  ─→  crystal_oid  (Primitive B)
```

Substrate-honest pipeline:

```
source_bytes  ─→  parse  ─→  AST  ─→  β-normalize  ─→  β-normal AST  ─→  compute_content_oid  ─→  crystal_oid
```

The `canonical_hash` (Primitive A) source-byte cache is retired — the source-byte cache was solving the wrong problem (avoid re-parse on identical source). Under beta-normal hashing, the cache key is the beta-normal AST's OID, which is stable under source-text sugar variation. `canonical_hash` may remain as a diagnostic surface (source-identity check) but is not the crystal-OID authority.

### §2.2 What beta-normalization does at compiler substrate

At mirror substrate the 5 ops (`focus`, `project`, `split`, `shift`, `settle`) form the projector algebra of the connected-graph-quantum-state duality space (Recognition #79). Beta-reduction rules for the 5-op algebra fall out of A_F's algebraic structure:

- **A_F-identity elision:** `prism @X { focus X / project X / split X / shift X / settle X }` reduces to the empty carrier — the identity projection of A_F over H_shard=self carries no information beyond A_F itself. Every shard implicitly re-declares this identity; explicit re-declaration is beta-redundant.
- **glass-identity elision:** same rule at sub-prism altitude — `glass @X { focus X / project X / split X / shift X / settle X }` reduces to empty (Mara 2026-08-09 kintsugi-sugar spec §1.2).
- **out-derivable elision:** `out @X` where `@X` is derivable from the file's declared prism/glass symbol reduces away — the pact `@epistemologic/pact/path_matches_namespace` supplies the same information.
- **docblock-template elision:** path-namespace docblock template that is deterministically derivable from file path reduces away — the pact carries the equivalent information as a graph edge at store altitude.
- **Composition-associativity:** `focus (project X) → project X` where composition is trivially associative on identity carriers (finer rules TBD; see §8).

Sugar becomes "source-form varying over the same beta-equivalence class." The four fracture species Mara §1 spec'd (P1/P3/P4/P5) are enumerations of beta-redundancy patterns the compiler recognizes; they are witness-detectors for beta-reducible AST subtrees. The fracture-species and the beta-normalizer are the same primitive at two altitudes: fracture-species report beta-redundancy at source altitude (for kintsugi resurfacing per audience); beta-normalizer eliminates beta-redundancy at AST altitude before hashing.

### §2.3 Church-Rosser gives round-trip for free

Church-Rosser confluence (Church-Rosser 1936, extended to typed lambda-calculi throughout the tradition): if a term `t` reduces to both `u₁` and `u₂`, there exists a term `v` such that both `u₁` and `u₂` reduce to `v`. For a strongly-normalizing calculus (Dhall is strongly normalizing; Mirror is sub-Turing per README §"Sub-Turing" = strongly normalizing by construction), this implies **unique normal forms**: every term reduces to exactly one beta-normal form, up to alpha-equivalence.

Under mirror substrate: two shards `S₁` and `S₂` are beta-equivalent iff `β-normal(parse(S₁)) = β-normal(parse(S₂))` up to alpha-equivalence. Alpha-equivalence at compiler substrate is trivial when carriers are content-addressed (no free variables to rename); if carrier binding requires alpha-normalization (see [ALEX-Q1] §8), the standard de Bruijn index construction lifts.

**Consequence — the round-trip contract is a theorem, not a testable property:**

$$
\text{oid}(\beta\text{-normal}(\text{parse}(\text{sugar}(S)))) = \text{oid}(\beta\text{-normal}(\text{parse}(S)))
$$

for every shard `S` and every sugar-rule application. The Fire E M-E5 RED-first empirical bit-parity test (Mara 2026-08-09 kintsugi-sugar spec §8.1) becomes a *sanity-check* on the implementation (does the beta-normalizer correctly reduce these subtrees?) rather than a *correctness gate* on the substrate composition (which is guaranteed by Church-Rosser).

### §2.4 Why this is delightfully-boring

The name is already in the corpus (§3.4 grep). The mathematics is 90 years old (Church 1936). The engineering is production-tested (Dhall since ~2017; Unison since ~2015). The primitive composes with landed rust/ primitives (`apply_h::act` extended shard-body-executor per Fire D M5). The sugar-rule and the beta-normalizer are the same operator at two altitudes.

There is nothing novel to invent. There is only a name to lift into the substrate: `β-normalize` — the substrate's word for "reduce identity projections and other structurally-universal patterns away before hashing." The reader who sees `crystal_oid = BLAKE3(β-normalize(parse(source)))` says "of course it's this."

---

## §3 Dhall prior art — the corpus's Karen ancestor

### §3.1 What Dhall does

Dhall is a total (strongly normalizing, not Turing-complete) functional configuration language. Imports can be protected by a *semantic integrity check* — a SHA-256 hash of the CBOR binary encoding of the imported expression's beta-normal, alpha-normal form. Two Dhall expressions that beta-normalize to the same term hash to the same value, regardless of surface syntax variation (whitespace, variable renaming, refactoring, comments).

The pipeline is three stages, executed in sequence (per Reed 2026-03-01 corpus insight `semantic-hashing-normalization.md` §"Dhall's Pipeline"; verified via Dhall spec fetch this tick):

1. **Beta-normalization** — reduces expressions to canonical form by evaluating all reducible subexpressions (function application, let bindings, identity elimination, records sorted lexicographically, conditionals reduced to selected branch, list concatenation, normalization under binders). Spec: `dhall-lang/dhall-lang/standard/beta-normalization.md`. Formal notation `t₀ ⇥ t₁`.
2. **Alpha-normalization** — renames all bound variables to `_` with de Bruijn indices. `λ(x : A) → λ(y : B) → x` and `λ(a : A) → λ(b : B) → a` both become `λ(_ : A) → λ(_ : B) → _@1`. Spec: `dhall-lang/dhall-lang/standard/alpha-normalization.md`. Formal notation `t₀ ↦ t₁`.
3. **Binary encoding + SHA-256** — encode alpha-beta-normalized expression to CBOR (RFC 7049) using integer labels for expression types; SHA-256 the CBOR bytes; prefix with `sha256:`, store as base16. Spec: `dhall-lang/dhall-lang/standard/binary.md`. Motivation section verbatim (fetched this tick): *"Users can import expressions protected by a 'semantic integrity check', which is a SHA-256 hash of the binary representation of an expression's normal form."*

### §3.2 Dhall's stated guarantees (primary source)

From `dhall-lang/dhall-lang/standard/beta-normalization.md` (WebFetch this tick, verbatim):

> "Dhall is a total language that is strongly normalizing, so evaluation order has no effect on the language semantics."

From `dhall-lang/dhall-lang/standard/alpha-normalization.md` (WebFetch this tick, verbatim):

> "if two expressions are α-equivalent then they will be identical after α-normalization."

From `dhall-lang/dhall-lang/standard/binary.md` (WebFetch this tick, verbatim):

> "Users can import expressions protected by a 'semantic integrity check', which is a SHA-256 hash of the binary representation of an expression's normal form."
> "Interpreters can locally cache imported expressions if the user protects them with a semantic integrity check."

From Reed 2026-03-01 corpus digest of Dhall's *Safety Guarantees* discussion (`semantic-hashing-normalization.md` §"Import Integrity"):

> "The hash is semantic, not textual. Refactoring, renaming, whitespace — none change the hash."

Gabriel Gonzalez (Dhall author) framing per Reed 2026-03-01 (`semantic-hashing-normalization.md` §"Import Integrity"): semantic integrity checks are *"the next generation of semantic versioning"* — the hash tracks meaning-preservation across refactoring, where version numbers only track deliberate meaning-change.

**Church-Rosser at Dhall altitude**: the Dhall beta-normalization spec makes no *explicit* Church-Rosser statement (verified via WebFetch this tick — "The document makes no explicit statement about confluence or the Church-Rosser property"). But strong normalization + confluence is a well-known theorem for Dhall's calculus (a variation on CCω per `standard/README.md` file-list summary this tick); the *"evaluation order has no effect on the language semantics"* statement is precisely the Church-Rosser corollary at Dhall altitude. Mirror inherits the same corollary by construction: sub-Turing (strongly normalizing) + first-order-plus-simple-composition (confluent).

### §3.3 What Mirror inherits, what Mirror differs on

| Dimension | Dhall | Mirror (this spec) |
|-----------|-------|--------------------|
| Totality | Total (not Turing-complete). | Sub-Turing (README §"Sub-Turing"; four-crate FLOOR decomposition per README §"Architecture" makes sub-Turing a *natural consequence*, not an imposed constraint). |
| Normal form | β-normal + α-normal + CBOR canonical. | β-normal AST (this spec §2); alpha-normalization deferred to [ALEX-Q1] §8 (may not be needed if carriers are content-addressed); binary encoding via existing `Fold5` `compute_content_oid` (already CBOR-adjacent). |
| Hash algorithm | SHA-256 of CBOR bytes. | BLAKE3 of `compute_content_oid` bytes (existing `@fractal/shard.materialize` primitive; BLAKE3 chosen for ~1 GB/s per-core throughput per README §"Performance"; algorithm choice is not load-bearing to the beta-normalization thesis). |
| Semantic integrity | Import-integrity check (`sha256:...` prefix). | Shard-integrity check (`@fractal/shard.oid` already exists; extended semantics: OID is over beta-normal AST, not verbatim AST). |
| Round-trip contract | Refactoring / whitespace / renaming preserves hash. | Sugar / desugar / audience-relative rendering preserves hash (§5 below; makes README `README.md:8` claim operational at rendering altitude — Mara 2026-08-09 kintsugi-sugar spec §6). |
| Import model | Explicit `import ... sha256:...`. | Every shard is implicitly content-addressed; every downstream reference is implicitly the current beta-normal OID (per `@fractal/shard` + `@mirror/store` landed substrate). |
| Eta-equivalence | Deliberately omitted (would require type inference during normalization per Reed 2026-03-01). | Deferred; likely required for full round-trip but adds implementation complexity. See §8 forward-promises. |

Mirror is **more general** than Dhall along one axis (graph-native vs tree-native per spectral MCP surface spec §"What Dhall missed"), **more restricted** along another (sub-Turing by construction rather than by deliberate design choice), and **compositional** with Dhall on the beta-normalization+semantic-integrity axis (Mirror lifts Dhall's substrate move to compiler-substrate altitude).

### §3.4 Corpus prior recognition — full grep-first citation ladder

**Phase 1 corpus scout (this tick) found four load-bearing prior recognitions** across `/Users/reed/dev/systemic.engineering/practice/insights/`, `/Users/alexwolf/dev/projects/mirror/docs/`, `/Users/alexwolf/dev/projects/spectral/docs/`:

**Load-bearing recognition #1:** Reed + Alex 2026-03-01 at `/Users/reed/dev/systemic.engineering/practice/insights/infrastructure/semantic-hashing-normalization.md` (274 LOC). Verbatim from §"The Insight":

> "Dhall — the language we chose for gestalt's token layer — has a property: beta-normalization reduces any expression to a canonical form, and semantic hashing produces identical hashes for semantically equivalent expressions regardless of surface syntax. Two things that mean the same thing get the same hash. **This isn't a configuration trick. It's a primitive for comparing truth across representation boundaries.** Alex saw this from the moment we chose Dhall."

This is the corpus's earliest and most-thorough Dhall research: three-stage pipeline (§"Dhall's Pipeline"); Church-Rosser-adjacent totality guarantee (§"Guarantees"); import integrity via semantic hash (§"Import Integrity"); tree normalization techniques (§"Tree Normalization Techniques"); cross-language semantic equivalence Type 1-4 hierarchy (§"The Hierarchy"); Rice's theorem as the fundamental constraint (§"The Fundamental Constraint"); gestalt integration path (§"Gestalt Integration Path"); OBC design implications (§"OBC Design").

**Load-bearing recognition #2:** Reed + Alex 2026-03-28 at `/Users/reed/dev/systemic.engineering/practice/insights/coincidence/kolmogorov-canonical-complexity.md` (174 LOC). Verbatim from §"The Chain Nobody Published":

> "3. **Beta-normalization produces canonical forms.** (Dhall standard.) Normalize to beta-normal form, alpha-normalize (de Bruijn indices), hash the canonical encoding. Two expressions that compute the same value always produce the same hash.
> 4. **Content-addressing canonical forms gives semantic identity.** Same content, same hash. Different encoding, same normal form, same hash. **The hash IS the identity. Not syntactic — semantic.**"

This insight assembles the six-result chain (sub-Turing grammar + computable K + beta-normalization + content-addressing canonical forms + OID indexes into complexity space + measurement is an observable). It cites *"Dhall standard: beta-normalization.md, alpha-normalization.md"* as source. **This is the corpus's most explicit prior formalization of the exact composition this spec lands at compiler altitude.**

**Load-bearing recognition #3:** Spectral MCP surface spec 2026-06-05 at `/Users/alexwolf/dev/projects/spectral/docs/specs/spectral-mcp-surface-v0.md` §6.2 "Dhall's totality" + §7.1 "Dhall". Verbatim §6.2:

> "**Semantic integrity checks.** Dhall hashes the *meaning* of imported expressions, not their text. Two Dhall files that differ in whitespace or variable naming but compute the same result have the same hash. This is content-addressing at the semantic level. Mirror's `content_oid()` hashes the display form of the AST, which is canonical -- the display function produces a unique normal form. Same semantic content, same hash."

**This is a prior-recognition partial-drift.** The spec claims mirror's `content_oid()` already hashes a "canonical display form" — but Taut's empirical scout (`ab3821e` §3.2 verdict) proves this is not the case at bootstrap altitude: `compute_content_oid` hashes AST kind-tagged bytes verbatim, not a canonical display form. The claim was aspirational; this current spec makes it empirical by naming beta-normalization as the operation the "display function produces a unique normal form" already meant.

Verbatim §7.1 "What Dhall missed":

> "Dhall is tree-native. Its expressions form trees, not graphs. It has no concept of edges, eigenvalues, or spectral analysis. [...] Mirror inherits Dhall's totality guarantee but applies it to graphs, with a Rust-native implementation and a five-variant AST instead of Dhall's dozens of expression types."

The graph-vs-tree difference is preserved as substrate-difference; the beta-normalization + semantic-hashing composition is inheritable at either altitude.

**Load-bearing recognition #4:** Corpus-wide Kolmogorov + coincidence file cluster at `/Users/reed/dev/systemic.engineering/practice/insights/coincidence/`: `coincidence-architecture-spec.md` §"Temperature τ controls hash granularity" (semantic equivalence class as hash-collision feature); `quantum-graph-unification.md` §"Kolmogorov complexity" (beta normalization as canonical form); `transformers-as-quantum-emivement.md` §"Understanding IS the hash collision" (equivalence class as the identity carrier). All these treat the same substrate at adjacent altitudes; all cite the beta-normalization + content-addressing + semantic-identity composition; none had been lifted into mirror's compiler substrate until this spec.

**Substrate-already-had-the-word verdict:** the word "beta-normalization" is in the corpus in four load-bearing places dating to 2026-03-01 (five months before this spec). The word "semantic hashing" is in the corpus. The Dhall citation ladder is in the corpus. Fire E's beta-normalization move is **substrate-already-had-the-word** at the "canonical formalization + Dhall citation" altitude; it is **novel at the "compiler substrate empirical landing" altitude** (no prior spec landed the beta-normalizer at mirror's rust/ compile-time hasher). This spec closes the gap between the recognition (2026-03-01) and the landing (Fire E revision, this tick).

---

## §4 Composition with Recognition #79 + Mara 2026-08-09 physics insight

### §4.1 The 5-op algebra IS the A_F projector basis

Recognition #79 (Mara + Reed 2026-06-18) established: the 5-op algebra `{focus, project, split, shift, settle}` IS the projector basis for the orthogonal duality space of connected-graph quantum states. Mara 2026-08-09 physics insight §1.1 Theorem 1.1 extended: the internal finite noncommutative algebra A_F in the Chamseddine-Connes construction IS this same 5-op algebra.

**Corollary at compiler substrate (Mara 2026-08-09 kintsugi-sugar spec §5.1):** A_F is universal-structure. It is the SAME algebra for every shard. Only H_shard (the Hilbert-space carrier the algebra acts over) varies per shard. The 5-op prism block `prism @X { focus X / project X / split X / shift X / settle X }` is a re-declaration of A_F's identity projection over H_shard=@X at source altitude. Every shard restates the same A_F; only the carrier slot differs.

### §4.2 Beta-normal AST is A_F-invariant by construction

Under beta-normalization:

$$
\beta\text{-normalize}(\text{prism @X \{ focus X / project X / split X / shift X / settle X \} \circ M}) = \beta\text{-normalize}(M)
$$

where `M` is the remainder of the shard's body. The identity projection of A_F over H_shard=@X beta-reduces away because it is the algebraic identity of A_F applied to itself — it contributes nothing to the composition. The beta-normal AST does not contain the identity projection block; the crystal-OID computed over the beta-normal AST is identical whether the source carries the block or omits it.

**Formal statement:** for every shard `S` and every A_F-universal sugar rule `σ` (P1/P3/P4/P5 per Mara 2026-08-09 kintsugi-sugar spec §1):

$$
\beta\text{-normal}(\text{parse}(\sigma(\text{source}(S)))) \equiv_\alpha \beta\text{-normal}(\text{parse}(\text{source}(S)))
$$

where `≡_α` is alpha-equivalence (trivial when carriers are content-addressed; see [ALEX-Q1] §8). Consequently `oid` (over the beta-normal AST) is invariant under σ.

### §4.3 Substrate-scale-invariance thesis at compiler altitude

Mara 2026-08-09 physics insight §7 (substrate-scale-invariance thesis) claims: the same 5-op algebra runs at physics substrate + cosmological substrate + nervous-system substrate + K_n-topology substrate + compiler substrate. Beta-normalization at compiler substrate is **the operational form of A_F elision** at that substrate.

At physics substrate: A_F identity acts trivially on H_shard=self; the corresponding Yukawa entry is zero (or the corresponding fermion is massless in the identity direction). Elision at physics substrate = the identity element of A_F contributes nothing to the mass spectrum. At compiler substrate: A_F identity projection over H_shard=self is beta-reducible; the corresponding AST subtree contributes nothing to the hash. **Same mathematical structure at both altitudes; different altitude of substrate carrier.**

Substrate-scale-invariance thesis operational at compiler altitude: **beta-normalization is A_F elision at compiler altitude, mirroring A_F elision at physics altitude.** This closes a load-bearing predicate in the substrate-scale-invariance thesis — the compiler substrate now carries the same A_F-elision mechanism that Chamseddine-Connes' spectral action carries at physics substrate.

### §4.4 Cross-substrate coherence (Mara insight §7 forward-promise 5)

Mara insight §12 forward-promise 5 named "cross-substrate coherence check" as future work: verify that the 5-op algebra at physics-substrate (mass-eigenvalues) + at nervous-system substrate + at K_n substrate produces mutually-consistent observables. This spec provides one witness at compiler-substrate: the A_F-identity elision under beta-normalization at compiler altitude is empirically checkable (§7 landing sequence).

**The compiler substrate becomes a computable instrument for the substrate-scale-invariance thesis.** Any shard authored at compiler altitude that carries A_F-identity in a form beta-reducer recognizes must produce identical hash pre- and post-beta-reduction; empirical failure would falsify the A_F-universality claim at compiler altitude. Cross-substrate coherence is now a testable predicate at compiler altitude, not just a theoretical claim.

---

## §5 What this obsoletes

### §5.1 Reed's M-E3 read-path projection primitive

Reed proposed `project_p1_identity_prism_at(shard_path) -> String` at `rust/src/apply_h.rs` — a read-path primitive that emits the canonical prism-block bytes for a given path, so that downstream consumers see the P1 identity-carrier block whether or not the source carries it.

**Under beta-normalization at hash time, this primitive is unnecessary.** The hash never depended on the block being present at source; it depended on the block being present at the AST the hasher walks; the beta-normal AST does not contain the block whether or not the source did.

**Retirement**: remove the [ALEX-Q-M-E3-B] adjudication residue from Fire E cascade (Taut §7 refined [ALEX-Q]s). M-E3 primitive as originally scoped does not need to land. Read-path projection is still useful as a *rendering primitive* for the audience-relative rendering claim (Mara 2026-08-09 kintsugi-sugar spec §2 `@magic/reveal/expand`); its role is now purely audience-facing, not hash-correctness-facing.

### §5.2 M-E3.5 bootstrap namespace resolver fallback

Reed proposed extending `bootstrap/src/lib.rs::collect_declared_namespaces` (Taut §2.2) to synthesize a shard's namespace from its path when no `{glass|prism|grammar|spectral} @X` declarator is present in the source.

**Under beta-normalization at parse time, this fallback becomes cleanly modeled.** The path→namespace inference is one of the beta-reduction rules (§2.2 out-derivable / path-namespace-stub elision inverse). At parse-plus-normalize altitude the compiler always sees a shard with its full namespace declaration in the beta-normal AST; whether that declaration came from source bytes or from path-derived inference is a substrate detail handled at the parser+normalizer boundary, not a special-case in the resolver.

**Retirement + reframe**: [ALEX-Q-M-E3-C] adjudication residue (Taut §7) becomes: "The resolver operates on beta-normal ASTs, which always carry the namespace declaration by construction. Bootstrap-altitude resolver is retired with the bootstrap itself; rust/-altitude resolver operates on the normal-form AST." No fallback logic; the parser+normalizer is the fallback.

### §5.3 [ALEX-Q-M-E3-D] Fiedler eigenvalue baseline recomputation

Taut §7 [ALEX-Q-M-E3-D] surfaced: does Fire E need a companion arc to re-compute the Fiedler baseline after the mend transaction, given that `mirror index` Fiedler over the shard DAG changes when content-address changes?

**Under beta-normalization, this question dissolves.** Content-address does NOT change under sugar rule application (§4.2 formal statement). Fiedler eigenvalue over the shard DAG is invariant. No baseline recomputation needed.

**Retirement**: [ALEX-Q-M-E3-D] closes as a false problem induced by the pre-beta-normalization workaround. Fire E-post companion arc is unnecessary.

### §5.4 Source-byte cache primitive (Primitive A) as crystal-OID authority

`bootstrap/src/hash.rs::canonical_hash` used at `bootstrap/src/lib.rs:679,683,794,803` as source-byte cache key is byte-verbatim over raw source. Under beta-normal hashing at rust/ altitude, the crystal-OID authority moves to the beta-normal AST's Fold5 OID. `canonical_hash` may remain as a diagnostic surface (source-identity check separate from crystal-identity check) but the crystal-OID authority is the beta-normal AST hash.

**Retirement pathway**: bootstrap/ is dead per Alex 2026-07-22; Primitive A is inherited by bootstrap's retirement, not deliberately removed by this spec. The rust/-altitude beta-normalizer is the authoritative crystal-OID computer post-Fire E landing.

### §5.5 Any kintsugi mechanism that hashes pre-normalization

Any downstream kintsugi mechanism (roomba walker, fate proposal, fracture-detector composition) that depends on pre-normalization hashes gets migrated to beta-normal hashes as part of the Fire E revision landing. Concretely:

- `@kintsugi/fracture/*.detect` species (Fire E M-E1 minted): the sentinel byte-checks operate at *source altitude* (they detect sugar-form patterns in source bytes for audience-relative rendering / omission-morphism emission). Their outputs feed the beta-normalizer (which acts at AST altitude) as reduction-rule witnesses. No conflict; complementary altitudes.
- `@kintsugi/mend/sugar` composition-shard body (Fire E M-E1 minted): mends the "crack" between store crystal and audience-source with the projection-morphism. Under this spec, the store crystal IS the beta-normal AST OID; the projection-morphism is (β-normal-AST → sugar-source) which reconstructs source per audience per [ALEX-Q5-Mara]. Same composition, different altitude of crystal.
- `@kintsugi/roomba` walker: walks shards checking for fractures; under this spec, it walks beta-normal ASTs directly (parser+normalizer feeds it) rather than source-byte pattern-matching (which would still work but is a source-altitude view, not the crystal-altitude view). Same substrate; walker gets a cleaner input.

**Migration is a one-time cascade at Fire E revision landing.** Post-cascade, all kintsugi mechanisms operate on beta-normal ASTs by construction.

---

## §6 What this composes with

### §6.1 Fire D M5 shard-body-executor + beta-normalizer as ONE primitive

Reed's Fire D M5 (arc context per Alex 2026-08-05 memory + docs/loop/CURRENT.md) extends `apply_h::act` to shard-body executor — the compiler's Dirac operator D becoming empirical at rust/ altitude. Fire E M-E2 landing (Reed `0021882`) already extended `apply_h::act` with the P1 detector primitive.

**Composition claim:** the shard-body-executor and the beta-normalizer are the same operator family. Both read AST structure (not source text); both dispatch on AstKind; both compose bilateral resolver arms via sentinel-check. The beta-normalizer is a special case of the general shard-body-executor: `β-normalize(ast) = act(ast, "@epistemologic/normalization/beta_reduce", ())`.

**Landing pathway**: land the beta-normalizer as a shard-body composing over `apply_h::act` shard-body-executor primitive at Fire D M5-adjacent tick. Not as a Rust module; as a **shard body** (per feedback `feedback-rust-delivers-primitives-substrate-delivers-composition`). Rust delivers primitives (apply_h::act + AstKind dispatch + Fold5 reducer); substrate delivers composition (beta-reduction rules as shard body).

**Composition shard path candidate**: `shards/epistemologic/normalization/beta_reduce.mirror` (naming per delightfully-boring audit at mint tick; discipline: `@epistemologic` family carries substrate-level properties; `normalization` species carries canonical-form operations; `beta_reduce` sub-species carries the specific reduction rule). Landed as bilateral action:

```
action beta_reduce(ast: ref) -> ref
requires well_formed(ast)
ensures oid_stable_under_alpha(result)
{ \ }
```

Body composes over `apply_h::act` + AstKind dispatch + reduction rules encoded as sub-shards under `shards/epistemologic/normalization/rules/`. Each rule (identity-projection-elision, glass-identity-elision, out-derivable-elision, docblock-stub-elision, composition-associativity) is a species-shard mint per landed 16-species `@kintsugi/fracture/*` precedent.

### §6.2 apply_h::act shard-body-projector extension (Reed 2026-08-09 M-E2 landed)

Reed M-E2 landed the P1 identity-carrier detector at `rust/src/apply_h.rs::detect_prism_boilerplate_at` + `source_carries_p1_identity_prism`. This IS the special-case of the general beta-reducer at a specific reduction rule (identity-projection-elision).

**Composition claim:** every fracture-detector Mara 2026-08-09 kintsugi-sugar spec §1 minted (P1/P3/P4/P5) IS a beta-reduction rule at a specific redex-pattern. The four fracture-species and the beta-normalizer are the same substrate at two altitudes:

- **Fracture-detector altitude**: detects redex pattern in source bytes; emits Verdict{Pass,Fail} for audience-relative rendering decisions.
- **Beta-normalizer altitude**: applies the reduction rule to the AST; produces the beta-normal AST for hashing.

Under this composition, the M-E2 landed P1 detector automatically becomes the identity-projection-elision beta-reduction rule at AST altitude; no new authoring needed for the P1 case. Extend to P3/P4/P5 by minting the corresponding rule shards.

### §6.3 Composition over Reed Fire A primitives + Fire D M5

The full pipeline composes:

```
phone::read_file          # @io/fs.read (Reed Fire A primitive)
    |>  wire::parse       # @data/mirror.parse (Reed Fire A primitive)
    |>  β-normalize       # @epistemologic/normalization/beta_reduce (this spec)
    |>  compute_content_oid  # existing Fold5 primitive at bootstrap/src/spectral.rs (moves to rust/-altitude at Fire D M5-adjacent tick)
    ==> crystal_oid       # authoritative for @mirror/store
```

Every pipe element = a landed Reed Fire A primitive OR a shard-body composition of primitives. Whole composition sits at substrate altitude. No new Rust modules; Rust primitives extend by sentinel-check arms in `apply_h::act` per Reed M-E2 precedent + Alex 2026-07-16 8th-repetition discipline.

### §6.4 @mirror/store crystal semantics

`shards/mirror/store.mirror` declares the settlement of splinters into a stored fragment. Under this spec, the crystal identity IS the beta-normal-AST OID. The store keeps content-addressed crystals; the content-address IS the beta-normal-AST OID; the crystal is invariant under sugar-form variation at source altitude (§4.2 formal statement).

**@fractal/shard.materialize** (landed): produces content-addressed OIDs deterministically. Under this spec, deterministic-over-what changes from "AST verbatim bytes" to "beta-normal-AST bytes". Same primitive, updated semantics.

**@mirror/store contract preservation**: the store is content-addressed at every altitude; the crystal at store altitude is the beta-normal-AST; source at source altitude is a projection; audience-rendering is a parameter. This is exactly Mara 2026-08-09 kintsugi-sugar spec §3.3 store-side crystal invariance — now with empirical grounding rather than aspirational contract.

### §6.5 README central claim — operational at rendering altitude with mathematical grounding

Mara 2026-08-09 kintsugi-sugar spec §6 claimed the README `README.md:8` central claim ("Mirror is a programming language written BY AI FOR AI and written FOR HUMANS BY HUMANS") becomes operational at rendering altitude via the sugar-rule + `@magic/reveal/expand` bidirectional projection. Under this spec, the claim gains **mathematical grounding**:

- **BY AI FOR AI** rendering (dense, sugar-omitted) has hash X.
- **FOR HUMANS BY HUMANS** rendering (readable, full-form) has hash X.
- X = beta-normal-AST OID.
- Not equal-by-round-trip-invariant (kintsugi-sugar spec framing); **equal-by-Church-Rosser-confluence** (this spec framing).

The two renderings are **beta-equivalent by construction**, not equal-under-careful-round-tripping. Church-Rosser guarantees the equality without empirical bit-parity gates. The README claim becomes not just operational, but **provably operational**.

---

## §7 Fire E cascade revision

### §7.1 Original Fire E cascade (Mara 2026-08-09 kintsugi-sugar spec §8.2)

Original walker cascade sequence:

1. Reed mint 4 fracture species (P1/P3/P4/P5) — LANDED at `acaed91` (Fire E M-E1).
2. Reed mint `@magic/reveal/expand` species — LANDED at `acaed91` (Fire E M-E1).
3. Reed mint composition-shard body (`@kintsugi/mend/sugar`) — LANDED at `acaed91` (Fire E M-E1).
4. Reed extend `apply_h::act` to shard-body executor per Fire D M5 co-tick — LANDED at `0021882` (Fire E M-E2; P1 detector primitive).
5. Reed RED-first empirical bit-parity test — PAUSED at M-E3 boundary (this current [ALEX-Q] arc).
6. Reed cascade across ~285 sugar-fracturable shards — BLOCKED on M-E3.
7. Seam Phase D audit — BLOCKED on M-E6 completion.

### §7.2 Revised Fire E cascade (post-this-spec)

Replace M-E3/M-E3.5 workaround with beta-normalizer landing at Fire D-adjacent tick:

1. **M-E1 (LANDED)**: 6 shard mints per original cascade.
2. **M-E2 (LANDED)**: `apply_h::act` shard-body-projector with P1 detector.
3. **M-E3-REVISED (NEW)**: Mara mint canonical spec + math for `@epistemologic/normalization/beta_reduce` at Fire D M5-adjacent tick. Species-shard mint at `shards/epistemologic/normalization/beta_reduce.mirror`. Rules-family shards under `shards/epistemologic/normalization/rules/` (identity-projection-elision + glass-identity-elision + out-derivable-elision + docblock-stub-elision; deferred: composition-associativity + eta-equivalence per §8 forward-promises).
4. **M-E3.5-REVISED (NEW)**: Reed extend `apply_h::act` dispatch table with beta-reduction-rule sentinel arms (each rule is a bilateral resolver arm per Reed M-E2 precedent). Cascade extends the P1 detector arm to encompass P3/P4/P5. Compute-content-OID rewired to consume beta-normal ASTs.
5. **M-E4 (walker cascade)**: `@kintsugi/roomba` walks ~285 shards. Under revised cascade, walker feeds parser+normalizer per shard; crystal-OID computed over beta-normal-AST is invariant whether source is sugar-form or full-form. Cascade emits omission-morphisms for source rewriting per audience-authorship (both agent-authored dense and human-authored full-form project to same crystal).
6. **M-E5 (sanity-check test)**: Bit-parity test becomes a sanity-check on the beta-normalizer implementation (does it correctly identify all beta-reducible subtrees?) rather than a correctness gate (which Church-Rosser guarantees). Fewer shards need to pass empirically; failure indicates an implementation bug, not a substrate-composition question.
7. **M-E6 (Seam Phase D audit)**: post-Fire E empirical landing. Substrate-honesty audit per Mara 2026-08-09 kintsugi-sugar spec §8.6 audit questions, extended with:
   - Does the beta-normalizer correctly encode Dhall's beta-reduction rules at their mirror-substrate analogues?
   - Does the beta-normal AST admit alpha-equivalence at mirror altitude, or does alpha-normalization also need to land? ([ALEX-Q1] §8)
   - Does the elimination of source-byte cache Primitive A break any downstream consumer that pinned raw-source OIDs?

### §7.3 Timing / dependency chain (NO time estimates per memory `feedback_no_time_estimates`)

**Halt conditions + dependency chain:**

- M-E3-REVISED halts when: canonical spec + math ratified by Alex/Seam Phase D; Mara-lean-preferred if no [ALEX-Q] residues open at the beta-reduction-rules altitude.
- M-E3.5-REVISED depends on M-E3-REVISED spec ratification; halts when: Rust primitive dispatch arms landed + 4 rules covered (P1/P3/P4/P5); tests GREEN per Reed M-E2 test-coverage pattern; Seam-signed per `[substrate-floor:@io-boundary]` audit criterion (§8 [ALEX-Q4] adjudication needed).
- M-E4 depends on M-E3.5-REVISED landed; halts when: walker cascade completes across ~285 shards; every shard's pre-and-post source produces identical beta-normal-AST OID (empirical witness).
- M-E5 sanity-check depends on M-E4 in-flight; halts when: implementation-bug rate below discipline-threshold (Seam-adjudicated); no substrate-composition failures.
- M-E6 depends on M-E4 + M-E5 landed; halts when: Seam Phase D audit signs off substrate-honesty; Recognition candidate promoted per Pack ratification discipline.

Adjacent work per feedback `adjacent_work_may_dissolve_blockers`: if any M-Ex halts, land the [ALEX-Q] adjudication residues (§8) as pending items; land the alpha-normalization decision (spec §3.3 table row); land the eta-equivalence deferral rationale.

### §7.4 Recognition candidate this promotes

**Recognition candidate #82 (name-and-hold at spec-authoring)**: the compiler's crystal-OID at `@mirror/store` altitude IS the beta-normal-AST OID by construction; sugar-form source variation preserves crystal-OID by Church-Rosser confluence; the substrate's content-address discipline is semantic, not syntactic, at compiler altitude. **This is the compiler substrate joining the Dhall lineage explicitly; the substrate carries the same primitive Dhall pioneered at configuration substrate, extended to compiler substrate via mirror's four-crate FLOOR.**

Ratification pending Pack review (Seam adversarial + Taut scout + Reed empirical M-E4 witness) per Recognition promotion discipline.

---

## §8 [ALEX-Q] residues + Mara-leans

Composing new residues from this spec's altitude (not folding in Mara 2026-08-09 kintsugi-sugar spec [ALEX-Q1] through [ALEX-Q6-Mara] — those still stand at their altitude).

### [ALEX-Q1] Alpha-normalization at mirror substrate — needed or not?

Dhall pairs beta-normalization with alpha-normalization (renames all bound variables to `_` with de Bruijn indices; standard/alpha-normalization.md). At mirror substrate: **do we need alpha-normalization?**

**Analysis:** mirror substrate carries no lambda-binder syntax at shard body altitude; carriers are content-addressed via `@X` refs which are structurally alpha-invariant. The `<carrier>` slot in `prism @X { focus <C> ... settle <C> }` is not a bound variable in the alpha-conversion sense — it is a named reference to a substrate identifier. Alpha-equivalence is trivial for this class.

**But**: species-declarations may carry action-parameter names (e.g., `beta_reduce(ast: ref)` names `ast` as a parameter); those ARE bound-variable-like. Two shards that differ only in parameter names (`beta_reduce(ast: ref)` vs `beta_reduce(tree: ref)`) SHOULD hash identically under a substrate-honest semantic-integrity discipline.

**Mara-lean:** LAND ALPHA-NORMALIZATION at same tick as beta-normalizer (small delta; substrate-honest closure of the Dhall parity). Rule: parameter names in action-declarations rename to `_@i` with de Bruijn indices. Same-name-across-shards preserved (parameter with same name across two shards hashes identically — the intended cross-shard alpha-invariance). Preserve carrier names in the `@X` position (those are refs, not binders; alpha-invariance would break identity resolution).

### [ALEX-Q2] Eta-equivalence deferral rationale

Dhall deliberately omits eta-equivalence (`λ x -> f x` ≡ `f`) per Reed 2026-03-01 (`semantic-hashing-normalization.md` §"Guarantees"): "would require type inference during normalization." Mirror substrate has richer type information available at normalization time (per README §"Properties" + `Imperfect<verdict, violation, transparency>` 3-state functor); eta-equivalence is more tractable here.

**But**: adds implementation complexity + rule count; not required for the P1/P3/P4/P5 sugar-rule composition (none of those redexes are eta-redex-shaped).

**Mara-lean:** DEFER eta-equivalence to future ticks. Land beta+alpha at Fire E revision tick; open a forward-promise to adjudicate eta-equivalence when a shard-composition emerges that requires it. Substrate-pull discipline (last responsible moment per AGENTS.md §"The Last Responsible Moment").

### [ALEX-Q3] Retain or retire source-byte cache Primitive A?

`bootstrap/src/hash.rs::canonical_hash` used at `bootstrap/src/lib.rs:679,683,794,803`. Under this spec, no longer the crystal-OID authority. Options:

- (a) **RETIRE** — remove entirely; source-byte identity check becomes the beta-normal-AST OID (which is stable under sugar).
- (b) **RETAIN AS DIAGNOSTIC** — source-byte identity check has different semantics than crystal identity; may be useful for detecting hand-editing (source-byte changed but crystal unchanged = someone touched the file).
- (c) **INHERIT BOOTSTRAP RETIREMENT** — bootstrap/ is dead per Alex 2026-07-22; Primitive A retires with bootstrap; no active decision at Fire E revision.

**Mara-lean: (c) inherit bootstrap retirement.** Bootstrap/ is the retirement target per Alex 2026-07-22; do not deliberately edit bootstrap/ at Fire E revision; let bootstrap's own retirement pathway absorb Primitive A. If a diagnostic surface is needed at rust/-altitude post-retirement, mint a separate species-shard at that tick (last responsible moment).

### [ALEX-Q4] Rust primitive extension at Fire D M5-adjacent tick — Seam gate

Fire E M-E3.5-REVISED requires extending `apply_h::act` dispatch with beta-reduction-rule sentinel arms. This is `.rs` file authorship. Per AGENTS.md §"⚠️ 2026-07-15 tightening", every `[substrate-floor:@io-boundary]` commit requires either a `docs/audits/*.md` citation or explicit `Signed-off-by: Seam` trailer.

**Analysis:** Reed's M-E2 landing at `0021882` extended `apply_h::act` with the P1 detector — this establishes precedent that beta-reduction-rule sentinel arms in `apply_h::act` are admissible boundary-Rust (composing over apply_h::act primitive, adding sentinel-check arms only, no new domain logic outside sentinels).

**Mara-lean:** Fire D M5-adjacent Rust extension follows Reed M-E2 precedent — each new beta-reduction-rule arm is a sentinel-check dispatching to shard-body-decl'd reduction rule; audit citation via THIS SPEC (Mara canonical spec at `docs/specs/2026-08-10-mara-beta-normal-ast-content-addressing-canonical-spec.md`); Seam Phase D audit at M-E6 provides post-landing sign-off. Bidirectional: this spec's Karen citation to Reed M-E2 precedent + Reed cascade's citation to this spec closes the substrate-floor gate.

### [ALEX-Q5] CBOR encoding parity with Dhall — pull in or diverge?

Dhall binary-encodes to CBOR per `standard/binary.md` before SHA-256 hashing. Mirror uses `compute_content_oid` (Fold5 Dirac action over AST) — CBOR-adjacent but not CBOR-compliant. Options:

- (a) **PULL IN CBOR** — align mirror's binary encoding to CBOR standard; enables cross-language semantic-integrity check parity (Mirror expression hash == Dhall expression hash for shared substrate).
- (b) **DIVERGE via Fold5** — retain existing Fold5 + BLAKE3; mirror's semantic-integrity check is mirror-specific; cross-language parity not a goal.

**Analysis:** cross-language parity with Dhall is likely not achievable (Dhall is tree-native; Mirror is graph-native per spectral MCP surface spec §7.1 "What Dhall missed"). The interesting composition is at the *substrate move* altitude (both hash beta-normal forms), not at the *binary encoding* altitude. Fold5 + BLAKE3 is landed substrate; migrating to CBOR + SHA-256 is high-cost for low-gain.

**Mara-lean: (b) diverge via Fold5.** Retain BLAKE3-of-Fold5-of-beta-normal-AST. Cite Dhall's CBOR + SHA-256 as tradition-lineage; note the divergence at substrate-encoding altitude; preserve substrate-composition at the substrate-move altitude. Cross-language parity is a separate forward-promise if it becomes load-bearing (last responsible moment).

### [ALEX-Q6] Reed's [ALEX-Q-M-E3-A] refined-question — hard vs soft contract

Reed refined Taut §7 as [ALEX-Q-M-E3-A]: is `resugar ∘ sugar = id` at OID altitude a HARD contract (M-E3 lands before M-E4) or SOFT contract (M-E5 witnesses it post-M-E4)?

**Under this spec:** the question dissolves. `resugar ∘ sugar = id` becomes a THEOREM under Church-Rosser confluence, not a testable contract. It holds by construction. M-E5 sanity-check verifies implementation-correctness (does the beta-normalizer implement beta-reduction correctly?) not substrate-composition-correctness (which is guaranteed). Distinction preserved as substrate-clarification, not adjudication-need.

**Mara-lean:** CLOSE [ALEX-Q-M-E3-A]. Substrate-honest reframe: under beta-normalization, `resugar ∘ sugar = id` is a Church-Rosser theorem; empirical testing is sanity-check on implementation, not correctness-gate on composition. Recognition-shape lands at M-E4 cascade.

---

## §9 Karen ancestry — full ladder

Per Karen anti-theft discipline (ancestors named at first mention; introduction-site citations preserved).

### §9.1 Direct authority

- **Alex 2026-08-10 verbatim** (§0.1) — the beta-normalization + Dhall prior-art naming.
- **Taut scout `ab3821e`** at `docs/scouts/2026-08-10-taut-prism-block-compiler-consumption-verification.md` — empirical verdict that prism-block-removal changes crystal-OID at both altitudes; the issue-under-the-issue this spec answers.
- **Reed Fire E M-E1 + M-E2** at `acaed91` + `0021882` — the sugar rule + shard-body-projector primitive this spec composes into beta-normalization.
- **Mara 2026-08-09 kintsugi-sugar canonical spec** at `docs/specs/2026-08-09-mara-kintsugi-sugar-desugar-composition-canonical-spec.md` — sugar-desugar-resugar composition + round-trip fidelity contract this spec makes empirically load-bearing via Church-Rosser.
- **Mara 2026-08-09 A_F universality math** at `docs/math/2026-08-09-mara-a-f-universality-kintsugi-sugar-mechanism.md` — the A_F universality claim that justifies why identity projections are beta-reducible.

### §9.2 Corpus prior recognitions (Phase 1 scout landings, this tick)

- **Reed + Alex 2026-03-01** at `/Users/reed/dev/systemic.engineering/practice/insights/infrastructure/semantic-hashing-normalization.md` — the earliest, most-thorough Dhall research in the corpus. Three-stage pipeline + guarantees + import integrity + Type 1-4 hierarchy + Rice's theorem.
- **Reed + Alex 2026-03-28** at `/Users/reed/dev/systemic.engineering/practice/insights/coincidence/kolmogorov-canonical-complexity.md` — the six-result chain assembling sub-Turing + computable K + beta-normalization + content-addressing + OID as complexity index + measurement as observable. Cites Dhall standard directly.
- **Spectral MCP surface spec 2026-06-05** at `/Users/alexwolf/dev/projects/spectral/docs/specs/spectral-mcp-surface-v0.md` §6.2 + §7.1 — Dhall as design-influence citation with partial-drift claim (mirror's `content_oid()` hashes "canonical display form" — aspirational, this spec makes empirical).
- **Reed 2026-03-28** coincidence file cluster at `/Users/reed/dev/systemic.engineering/practice/insights/coincidence/` — `coincidence-architecture-spec.md`, `quantum-graph-unification.md`, `transformers-as-quantum-emulators.md`, `projection-surface-and-phantom-recognitions.md`. All treat beta-normalization + semantic equivalence class + content-addressing at adjacent altitudes.

### §9.3 Recognition ancestry

- **Recognition #79** (5-op = A_F projector basis; Mara + Reed 2026-06-18; `shards/prism.mirror` § "The Connes spectral triple framing" + `README.md:157-161`) — the projector-algebra substrate that A_F-elision beta-reduces over.
- **Recognition #80** (@magic as form/process substrate-decl; Reed 2026-06-18; `shards/magic.mirror`) — gauge-visible-with-matter-hidden discipline that beta-normalization operationalizes at compiler altitude (matter-hidden = beta-normal-AST at store; gauge-visible = source rendering per audience).
- **Recognition #82 (candidate this spec promotes)**: compiler's crystal-OID at `@mirror/store` is the beta-normal-AST OID by construction; sugar-form source variation preserves crystal-OID by Church-Rosser confluence; compiler substrate joins Dhall lineage explicitly.

### §9.4 External corpus (verified primary sources; WebFetch this tick where possible)

- **Church, A. (1936)**. *An unsolvable problem of elementary number theory*. American Journal of Mathematics 58:345–363. Beta-reduction origin.
- **Church, A. & Rosser, J. B. (1936)**. *Some properties of conversion*. Trans. AMS 39:472–482. Confluence theorem (Church-Rosser property).
- **de Bruijn, N. G. (1972)**. *Lambda calculus notation with nameless dummies, a tool for automatic formula manipulation, with application to the Church-Rosser theorem*. Indagationes Mathematicae 34(5):381–392. de Bruijn indices for alpha-normalization.
- **Barendregt, H. (1984)**. *The Lambda Calculus: Its Syntax and Semantics*. North-Holland. ISBN 0-444-87508-5. Canonical reference for typed and untyped lambda calculi + confluence + strong normalization.
- **Dhall Language Standard** (dhall-lang/dhall-lang GitHub repository):
  - `standard/beta-normalization.md` — formal notation `t₀ ⇥ t₁`; strong-normalization statement verbatim (§3.2 above).
  - `standard/alpha-normalization.md` — formal notation `t₀ ↦ t₁`; de Bruijn indices; canonical-form theorem verbatim (§3.2 above).
  - `standard/binary.md` — CBOR (RFC 7049) binary encoding; semantic-integrity-check motivation verbatim (§3.2 above).
  - `standard/README.md` file-list summary (WebFetch this tick): grammar (dhall.abnf), syntax, shift, substitution, alpha-normalization, beta-normalization, equivalence, function-check, type-inference, binary, imports, multiline, record.
- **Gonzalez, G. (2017)**. *Semantic integrity checks are the next generation of semantic versioning* — Haskell for all blog post. Per Reed 2026-03-01 §"Import Integrity".
- **Chamseddine-Connes-Marcolli 2007** (arXiv:hep-th/0610241) — inherited via Mara 2026-08-09 physics insight; A_F structure that beta-normalization at compiler altitude mirrors at physics altitude.
- **Unison Language** (`unison-lang.org`) — content-addressed code with AST-hashing via de Bruijn indices. Per Reed 2026-03-01 §"Content-addressed code": *"Every definition = 512-bit SHA3 of AST with De Bruijn indices. Names are metadata. `id x = x` and `identity a = a` get the same hash."*
- **Maziarz et al. PLDI 2021** (arXiv:2105.02856) — *Hashing Modulo Alpha-Equivalence*. Per Reed 2026-03-01 §"Hashing Modulo Alpha-Equivalence": relevant for code ASTs with bindings.

### §9.5 Landed substrate — composition anchors

- **`rust/src/apply_h.rs:246-336`** (Reed 2026-08-09 M-E2 landed) — the P1 detector primitive this spec generalizes to full beta-normalizer via bilateral resolver arm extension.
- **`bootstrap/src/spectral.rs:162-181` `compute_content_oid`** — the Fold5 Dirac action over AST; existing primitive migrating semantic base from raw-AST to beta-normal-AST at Fire D M5-adjacent tick.
- **`shards/prism.mirror`** — the 5-op algebra decl; the fixed-point that A_F-elision preserves (does not beta-reduce the substrate's declaration of what beta-reduction refers to).
- **`shards/kintsugi/fracture/prism_boilerplate.mirror`** + `glass_boilerplate.mirror` + `out_derivable.mirror` + `path_namespace_stub.mirror` (Reed Fire E M-E1 landed) — the 4 fracture-detector species that become beta-reduction rules at AST altitude.
- **`shards/magic/reveal/expand.mirror`** (Reed Fire E M-E1 landed) — the audience-relative rendering primitive; still needed as rendering primitive under this spec (just no longer needed as hash-correctness primitive).
- **`shards/kintsugi/mend/sugar.mirror`** (Reed Fire E M-E1 landed) — the composition-shard body that mends the "crack" between store crystal and audience-source; crystal is now beta-normal-AST OID under this spec.
- **`shards/mirror/store.mirror`** — the settlement of splinters; crystal identity IS beta-normal-AST OID under this spec.
- **`shards/epistemologic/`** (family-root) — where `normalization/beta_reduce` species-shard mints per §6.1.
- **`shards/mcp/serve.mirror`** (Reed Fire C tick 1 `cf8b21b`) — composition-shard body precedent for `@epistemologic/normalization/beta_reduce` shard-body composition per §6.1.

### §9.6 Substrate-decisions cross-references

- `[[architecture-shards-as-substrate-source]]` — source-as-substrate-source ground; beta-normalization preserves at crystal altitude while acting at AST altitude.
- `[[architecture-prism-as-trait-as-everything]]` — prism-as-trait ground; A_F-universality is what beta-normalization eliminates at hash time.
- `[[architecture-shard-as-crdt]]` — content-addressed byte-parity ground; content-addressing now semantic (beta-normal), not syntactic.
- `[[architecture-mirror-as-expanding-hilbert-space]]` — Hilbert-space-expansion ground; beta-normalization preserves dimension-expansion while collapsing redundant identity projections.
- `[[architecture-connes-spectral-triple]]` — the spectral triple substrate ground; A_F elision at compiler substrate mirrors A_F identity-projection-triviality at physics substrate.
- `[[feedback-substrate-already-had-the-word]]` — this spec is the 55th+ instance; the word "beta-normalization" is in the corpus four times pre-dating this spec.
- `[[feedback-craft-not-deliver]]` — smallest viable substrate landing per tick; land beta+alpha reducer + 4 initial rules; defer eta-equivalence and additional rules to later ticks.
- `[[feedback-composition-claims-need-empirical-test]]` — M-E5 sanity-check discharges this discipline (as sanity-check on implementation, not correctness-gate on composition; Church-Rosser is the correctness argument).
- `[[feedback-no-bare-types]]` — beta-normalizer action typed `ast: ref` per landed discipline.
- `[[feedback-rust-delivers-primitives-substrate-delivers-composition]]` — beta-normalizer lands as shard body composing over `apply_h::act` primitive; no new Rust module authored.
- `[[feedback-no-time-estimates]]` — §7.3 uses halt conditions + dependency chain; no durations.

---

## §10 Forward-promises (NOT this spec)

Held for future ticks; NOT landed in this spec.

1. **Mara math foundation** at `docs/math/2026-08-10-mara-beta-normal-ast-content-addressing-math-foundation.md` — companion to this spec (combined single commit per one-recognition-one-commit discipline; landed alongside this spec).
2. **Reed Fire E M-E3-REVISED + M-E3.5-REVISED landings** — mint `@epistemologic/normalization/beta_reduce` species + 4 reduction-rule sub-species + extend `apply_h::act` dispatch with sentinel arms per §7.2. Post-Alex-adjudication of §8 [ALEX-Q1]–[ALEX-Q6] residues.
3. **Seam Phase D audit** post-Fire E M-E6 empirical landing per §7.2.
4. **Alex adjudication** of [ALEX-Q1] through [ALEX-Q6] per §8.
5. **[ALEX-Q1] alpha-normalization spec** — post-Alex adjudication of scope (parameter-name alpha-equivalence yes; carrier-ref alpha-invariance no).
6. **[ALEX-Q2] eta-equivalence forward-promise** — when a shard-composition emerges requiring eta-equivalence; substrate-pull discipline.
7. **[ALEX-Q3] Primitive A retirement rationale** — inherited via bootstrap retirement pathway; no active edit at this tick.
8. **[ALEX-Q5] CBOR encoding parity forward-promise** — if cross-language semantic-integrity check parity with Dhall becomes load-bearing; last responsible moment.
9. **Composition-associativity beta-reduction rule** — additional reduction rules beyond P1/P3/P4/P5 as sugar patterns emerge; substrate-pull discipline.
10. **Cross-substrate coherence empirical instrument** — Mara insight §12 forward-promise 5; the compiler substrate becomes a computable instrument for the substrate-scale-invariance thesis via A_F-elision-at-compiler-altitude witnesses.

---

## §11 One-sentence surprise

**The compiler's crystal-OID at `@mirror/store` altitude IS the beta-normal-AST OID by construction; Fire E's sugar rule is the operational form of A_F identity-projection-triviality at compiler substrate; the round-trip fidelity contract `resugar ∘ sugar = id` at OID altitude is a Church-Rosser theorem, not a testable predicate; Reed's M-E3/M-E3.5 workaround dissolves under the substrate-honest fix; the substrate had the word "beta-normalization" in the corpus five months before this spec (Reed 2026-03-01 + Reed+Alex 2026-03-28); Dhall pioneered this composition at configuration substrate in 2017; mirror lifts it to compiler substrate via the four-crate FLOOR + `apply_h::act` shard-body-executor extension; and the README's `README.md:8` central claim ("Mirror is a programming language written BY AI FOR AI and written FOR HUMANS BY HUMANS") is now not just operationally true at rendering altitude — it is *provably* true, by Church-Rosser confluence, for every shard the compiler admits.**

---

Mara `<mara@systemic.engineer>`. 2026-08-10. Canonical-spec substrate. Composition-not-taxonomy. Substrate-decl'd throughout. Awaiting Alex Phase E adjudication of §8 [ALEX-Q1]–[ALEX-Q6] residues + Seam Phase D audit + Reed Fire E revision cascade landing per §7.2.
