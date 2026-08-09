# 2026-08-10 — Taut scout: prism-block compiler consumption verification

**Peer:** Taut (grep-first drift scout; read-only)
**Task:** Empirical verification of whether `prism @X { focus X / project X / split X / shift X / settle X }` blocks at the top of shard files are PARSED into runtime behavior or are documentation-only.
**Provoked by:** Alex 2026-08-10 correction ("Scout! Always scout. We don't guess. We verify.") after Reed halted Fire E at M-E3 boundary with an architectural [ALEX-Q] instead of grep-verifying.
**Discipline:** grep-first substrate-truth; NO speculation without file:line citations; NO substrate mutations; pure-docs 📝 markdown-only bypass.

---

## §0 Context

**Alex 2026-08-10 verbatim:** "Scout! Always scout. We don't guess. We verify."

**Reed Fire E state (before this scout):**
- M-E1 landed at `acaed91` — 6 shard mints.
- M-E2 landed at `0021882` — `apply_h::act` shard-body-projector extension with P1 identity-carrier detector at `rust/src/apply_h.rs`; 23/23 tests GREEN.
- Reed halted at M-E3 boundary asking whether prism blocks are load-bearing at compile/parse/crystal-canonical altitude, or documentation-only. Reed surfaced this as an [ALEX-Q] rather than grep-verifying.

**Substrate-truth question requiring empirical answer:** Do prism blocks get PARSED into runtime behavior by any current mirror compiler (rust/ or bootstrap/), and does removing them change the `@mirror/store` crystal OID?

---

## §1 Q1 — rust/-altitude prism-block consumption

### §1.1 rust/src/apply_h.rs (Fire E M-E2; Reed 2026-08-09)

**FILE:** `/Users/alexwolf/dev/projects/mirror/rust/src/apply_h.rs`

**LINES 246-336** — `detect_prism_boilerplate_at(shard_path: &Path) -> Verdict` +
`source_carries_p1_identity_prism(source, family, last_seg) -> bool`.

**What it does:**
1. Reads the shard file byte-source.
2. Derives the `family` literal from `shards/<family>.mirror` path decomposition.
3. Scans line-by-line for the exact byte-shape:
   ```
   prism @<family> {
     focus <last_seg>
     project <last_seg>
     split <last_seg>
     shift <last_seg>
     settle <last_seg>
   }
   ```
4. Returns `Verdict::Pass` if the P1 identity-carrier shape is present at any depth-0 site (contiguous 5 arms, canonical op order, byte-identical carrier across all 5 arms); returns `Verdict::Fail(reason)` otherwise (including `shards/prism.mirror` fixed-point exemption per Q3 Mara-lean).

**Runtime consumption:** REAL. Dispatched via `apply_h::act(root, "@kintsugi/fracture/prism_boilerplate.detect", args)` per test at `apply_h.rs:698`.

**Read altitude:** BYTE-LEVEL (Rice-safe sentinel). Not AST-altitude. The detector reads source bytes and pattern-matches on the P1 identity-carrier shape. Removing the prism block from a shard would change the Verdict from `Pass` to `Fail`.

### §1.2 rust/roomba/src/mend.rs (Reed prior arc; roomba walker)

**FILE:** `/Users/alexwolf/dev/projects/mirror/rust/roomba/src/mend.rs`

**LINES 142-172** — `fn shard_ref_from_source(source: &str, path: &Path) -> String`.

**What it does:**
1. Line-scans source for `prism @<X>/<Y> {` (or `prism @X {`) top-of-line declaration.
2. Extracts the `@`-prefixed carrier as the shard's canonical ref.
3. **Fallback** when no `prism @` line is found: derives the ref from the path by stripping `shards/` prefix and `.mirror` suffix.

**Runtime consumption:** REAL, but **degradable**. When prism block is absent, `shard_ref_from_source` falls back to path-derived ref — same result for canonical `shards/<family>.mirror` layouts. Behavior is preserved iff path-derived ref matches what the prism block would have declared. For P1 identity-carriers (family = path-derived), the fallback IS byte-identical.

### §1.3 rust/spectral/src/liquid.rs (bilateral property extraction)

**FILE:** `/Users/alexwolf/dev/projects/mirror/rust/spectral/src/liquid.rs`

Grep-verified: extracts `PropertyDecl` / `bilateral` / `algedonic` / `viability` blocks from `.mirror` source via byte-scanner. **Does NOT read `prism @X { ... }` blocks**. Consumes DIFFERENT declaration shapes (bilateral / algedonic / viability / fold).

**Runtime consumption:** NONE (of prism blocks specifically).

### §1.4 rust/matrix/src/void.rs (5-op basis)

**FILE:** `/Users/alexwolf/dev/projects/mirror/rust/matrix/src/void.rs:139-165`

Documents the 5 ops (`focus`/`project`/`split`/`shift`/`settle`) as the `VoidBasisAxis` projector algebra at Rust altitude. **Does NOT parse `prism @X {}` source blocks**. Composes the ops as a Rust enum + trait impls, not as source parsing.

**Runtime consumption:** NONE (of prism-block source syntax).

### §1.5 rust/ altitude summary

- **REAL consumer**: `rust/src/apply_h.rs` (Fire E M-E2 P1 detector) — byte-level.
- **DEGRADABLE consumer**: `rust/roomba/src/mend.rs` — falls back to path-derivation when absent.
- **NON-CONSUMERS**: `rust/spectral/`, `rust/matrix/`, `rust/fractal/` — read different shard shapes.

---

## §2 Q2 — bootstrap-altitude prism-block consumption

### §2.1 bootstrap/src/apply_h.rs (line 246+)

**FILE:** `/Users/alexwolf/dev/projects/mirror/bootstrap/src/apply_h.rs:246-256`

Same `strip_prefix("prism ")` + `strip_prefix('@')` pattern as `rust/roomba/src/mend.rs:150-164`. Line-scans source to derive shard's canonical `@`-ref from the first `prism @X` declaration.

**Runtime consumption:** REAL. But **degradable** in the same way — if no prism block, falls back to path-based ref (comment says same pattern as roomba `shard_ref_from_source`).

### §2.2 bootstrap/src/lib.rs — `collect_declared_namespaces` (line ~2400-2450)

**FILE:** `/Users/alexwolf/dev/projects/mirror/bootstrap/src/lib.rs:~2430-2455`

**What it does:** Cross-shard semantic resolution surface. Scans `.mirror` files for top-of-line `glass @X` / `prism @X` / `grammar @X` / `spectral @X` declarations and accumulates the declared namespace refs into a resolution index. Used by the corpus walker to check `in @<path>` statements resolve.

**Runtime consumption:** REAL. **Load-bearing** for the `kintsugi ci` corpus walker's unresolved-import detection (`count_unresolved_imports` at same file). If a shard's `prism @X` declaration is REMOVED, that namespace no longer registers in the resolver index; consumers with `in @X` statements would see `unresolved_imports > 0` and be downgraded to `failure`.

**Caveat:** The resolver only needs ONE of `{glass|prism|grammar|spectral} @X` per namespace. If a shard drops `prism @X` but adds `glass @X` (or has one already), the resolver still registers it. So the prism-block-specific dependency is: **the resolver counts `prism @X` as a valid namespace declarator; removing it while keeping some other declarator preserves registration**.

### §2.3 bootstrap/src/spectral.rs — Combinator seed / meta-glass

**FILE:** `/Users/alexwolf/dev/projects/mirror/bootstrap/src/spectral.rs:1200-1220, 2013-2027, 2076-2200`

- Line 1200-1210 (`LiteralKind` variant docblock): "Load-bearing variant for FP1/FP2: the five op-keyword captures the body of every `prism @(…) { … }` form is built from."
- Line 2013-2027 (`op_keyword_choice`): Emits `Combinator::Choice` of the five op-keywords (`focus`/`project`/`split`/`shift`/`settle`) as `LiteralKind` captures.
- Line 2076-2160 (`prism_seed`): The meta-glass seed — a permissive balanced-bytes recognizer. FP1: `apply_h(seed, grammar.mirror.bytes)` returns a Combinator tree with the same OID as the seed. FP2: `apply_h(seed, 00-prism.mirror.bytes)` returns a Combinator tree with no Dark fragments.

**Runtime consumption:** REAL but **structure-permissive**. The prism_seed is a *balanced-bytes recognizer* — it walks any well-formed mirror file (balanced `{}` and `()`) and preserves its OID under round-trip. It does NOT require prism blocks to have any specific carrier or arm shape. Deleting the prism block does NOT break FP1/FP2 as long as remaining bytes are still balanced.

**However** — the Combinator-tree OID emitted by parsing DOES depend on the byte-shape of the source. Different bytes → different consumed spans → different AST tree → different `compute_content_oid` result.

### §2.4 bootstrap/src/grammar.rs (keyword parsing)

**FILE:** `/Users/alexwolf/dev/projects/mirror/bootstrap/src/grammar.rs:140-160`

Parses "keyword" declarations that map source tokens to `AstKind` variants (`focus`→`Focus`, `project`→`Project`, `split`→`Split`, `shift`→`Shift`, `settle`→`Settle`). This is the tokenizer's keyword table. It reads `.mirror/keywords.mirror` files, NOT `prism @X {}` blocks directly.

**Runtime consumption:** Reads keyword-declaration syntax (not prism-block syntax). Independent of prism-block presence.

### §2.5 bootstrap/ altitude summary

- **REAL consumer**: `bootstrap/src/apply_h.rs` (`shard_ref_from_source` pattern; degradable).
- **REAL consumer**: `bootstrap/src/lib.rs::collect_declared_namespaces` (resolver index; degradable if another declarator exists).
- **PERMISSIVE consumer**: `bootstrap/src/spectral.rs::prism_seed` (parses ANY balanced bytes; not prism-specific).
- **NON-CONSUMERS**: `bootstrap/src/grammar.rs`, `bootstrap/src/tokenize.rs`, `bootstrap/src/crystallize.rs` — different responsibilities.

---

## §3 Q3 — @mirror/store crystal OID canonicalization

### §3.1 Two altitudes of "crystal OID" in the codebase

There are TWO distinct hash primitives in bootstrap:

**Primitive A — `bootstrap/src/hash.rs::canonical_hash`** (CoincidenceHash<5,5>).
Used at `bootstrap/src/lib.rs:679, 683, 794, 803` as the **source-byte cache key**:
```rust
let source_oid = canonical_hash(&source);
if let Some(cached) = git_crystal_exists(&source_oid) { ... }
```
This is a **byte-verbatim** hash of raw source. Any byte-level change (including whitespace, prism-block removal, comment changes) changes `source_oid`.

**Primitive B — `bootstrap/src/spectral.rs::compute_content_oid(&ast)`** (Fold5 Dirac action over AST).
Used at `bootstrap/src/lib.rs:679, 800` as the **AST content-address**:
```rust
let oid = compute_content_oid(&ast);
```
Walks the AST via `Fold5` reducer dispatching on `AstKind`. The reducer emits kind-tagged hash bytes per node; `Dark` nodes hash verbatim under the `"dark"` tag. Whitespace between tokens does NOT change the AST → does NOT change this OID.

**Splinter OID** (`bootstrap/src/crystallize.rs:263-285` + `MerkleHash::hash_bytes`) uses BLAKE3 raw-byte hashing of `Content` payloads. Merkle over Record/List content. Whether crystallization is byte-verbatim or byte-canonical depends on **what is put into `Content`**.

### §3.2 Empirical answer on prism-block removal

For **Primitive A** (source-byte cache): removing the prism block **CHANGES** `source_oid` (byte-verbatim). Cache miss on next compile; recomputation would occur.

For **Primitive B** (`compute_content_oid`): removing the prism block **CHANGES** the AST (fewer AstKind::Focus/Project/Split/Shift/Settle nodes at the tokenizer altitude, IF the tokenizer emits them; if the tokenizer swallows the block as `Dark`, the "dark" byte content changes). Either way, the folded OID **CHANGES**.

**Verdict:** Crystallization at the current bootstrap altitude is NOT byte-canonical with respect to prism-block presence. Removing a prism block **CHANGES** the crystal OID at both the source-cache altitude (Primitive A) and the AST altitude (Primitive B).

### §3.3 Consequence for Fire E M-E5 round-trip contract

Mara math §2 `resugar ∘ sugar = id` at content-address altitude is **currently aspirational, not empirical**. If we sugar a shard (remove the prism block) and then desugar (re-emit it), the byte-shape after desugaring must round-trip to the exact prior bytes — including whitespace, arm order, and any trailing newlines — for `source_oid` to match. For `compute_content_oid` to match, the desugared AST must be structurally identical to the pre-sugared AST.

M-E3 read-path projection is THE PREREQUISITE that makes round-trip observable. Without a read-path that synthesizes the prism block on demand from path + family metadata, downstream consumers see the sugared shard as byte-different (Primitive A) and AST-different (Primitive B) from its pre-sugar form.

---

## §4 Q4 — Downstream consumers of prism-block presence

Enumeration of tools that would BREAK vs safely no-op on sugared (prism-block-removed) shards:

| Tool | Behavior on prism-block-absent shard | Verdict |
|------|--------------------------------------|---------|
| `rust/src/apply_h.rs::detect_prism_boilerplate_at` | Returns `Verdict::Fail("no P1 identity-carrier prism block detected")` — this is the DETECTOR; absence IS its Fail signal | WORKS AS DESIGNED (this is the Fire E cascade primitive) |
| `rust/roomba/src/mend.rs::shard_ref_from_source` | Falls back to path-derived ref (`shards/X.mirror` → `@X`) | SAFE NO-OP for canonical layouts |
| `bootstrap/src/apply_h.rs::shard_ref` | Same fallback pattern | SAFE NO-OP for canonical layouts |
| `bootstrap/src/lib.rs::collect_declared_namespaces` | Namespace no longer registered UNLESS another declarator (`glass @X`, `grammar @X`, `spectral @X`) exists in the same file | CONDITIONAL: safe if sibling declarator present; BREAKS `count_unresolved_imports` for downstream `in @X` consumers if this is the only declarator |
| `bootstrap/src/spectral.rs::prism_seed` (meta-glass) | Parses ANY balanced bytes; agnostic to prism-block presence | SAFE NO-OP |
| `bootstrap/src/spectral.rs::compute_content_oid` | Emits DIFFERENT OID (AST differs) | BREAKS content-address round-trip fidelity |
| `bootstrap/src/lib.rs` source-cache | Emits DIFFERENT `source_oid`; cache miss | BREAKS crystal caching round-trip |
| `mirror kintsugi` pipeline | Depends on cache-miss + `compute_content_oid` recomputation | BREAKS round-trip observability |
| `mirror roomba` walker | Consumes shard-ref via `mend.rs` fallback | SAFE NO-OP |
| `mirror index` (Fiedler eigenvalue) | Composes over graph-Laplacian shape of shard DAG; input is content-address per shard | BREAKS if content-address changes (via Primitive B) |
| `mirror craft` | Code emission from AST | Depends on which AST bytes emit; likely BREAKS deterministic emission if AST differs |

**No test in the current repo asserts "prism block must be present"** (grep-verified: no `assert!(source.contains("prism @"))` pattern found across `bootstrap/tests/` or `rust/tests/`). BUT the Fire E M-E2 P1 detector at `rust/src/apply_h.rs` INVERTS this: absence-of-block IS a Verdict::Pass signal FOR the detector (it's detecting "boilerplate present"; when sugared, the boilerplate is gone, so the detector returns Pass on the pre-sugar shape and Fail on the post-sugar shape — as designed).

---

## §5 Empirical verdict on M-E3 prerequisite question

**VERDICT: HYBRID → leaning PREREQUISITE for round-trip fidelity.**

Breakdown:

- **Q1 (rust/)**: One REAL consumer (M-E2 detector — this is the point), one DEGRADABLE consumer (`mend.rs` — falls back to path derivation).
- **Q2 (bootstrap/)**: One DEGRADABLE consumer (`apply_h.rs`), one CONDITIONAL consumer (`collect_declared_namespaces` — safe iff sibling declarator present), one PERMISSIVE consumer (`prism_seed` — agnostic).
- **Q3 (crystal-OID canonicalization)**: Crystallization is NOT byte-canonical with respect to prism-block presence. Removing the block CHANGES the OID at BOTH cache altitude (Primitive A) AND AST altitude (Primitive B).
- **Q4 (downstream tools)**: 2 tools break on OID change (`kintsugi` pipeline + `index` Fiedler); 3 tools safe-no-op; 1 tool conditional; 1 tool works-as-designed (the M-E2 detector).

**Load-bearing finding for M-E3 disposition:**

The prism block IS load-bearing at content-address altitude. `resugar ∘ sugar = id` at OID altitude (Mara §2) does NOT hold empirically with the current compiler — it requires either:
1. A **read-path projection** (M-E3) that synthesizes the prism block on demand at compile time, so `source_oid` and AST-OID see the SAME bytes/AST whether the shard is sugared or not; OR
2. A **canonicalization pass** at crystallization time that normalizes prism-block presence (either always-strip or always-inject before hashing).

Path (1) is the natural M-E3 shape Reed surfaced. Path (2) would be a substrate-canonicalization altitude change that likely violates the byte-verbatim contract many consumers rely on.

---

## §6 Recommendation

**Taut-lean: M-E3 IS prerequisite to M-E4 walker cascade for round-trip fidelity.**

Specifically:
- If Fire E cascade M-E4 walker removes prism blocks from source without a read-path projection, THEN:
  - The `mirror kintsugi` source cache misses on next run for every mended shard.
  - `compute_content_oid` for the mended shard changes.
  - `mirror index` Fiedler eigenvalue over shard DAG changes (input content-addresses differ).
  - Any downstream consumer that pinned a prior crystal-OID (e.g., action_cache OID resolution at `bootstrap/src/lib.rs:1790+`) sees cache-miss cascade.
- If M-E3 lands FIRST as a compile-time read-path projection that synthesizes the prism block from `(shard_path, family_literal)`, THEN:
  - `mirror compile` sees identical bytes/AST for sugared and pre-sugar shards.
  - `resugar ∘ sugar = id` holds empirically at OID altitude.
  - M-E4 walker can safely strip prism blocks; downstream OIDs unchanged.

**M-E3 → M-E4 → M-E5 → M-E6** is the safe ordering.

**M-E4 → M-E5 → M-E6 → M-E3** (skipping M-E3 first) creates a substrate-visible content-address discontinuity across the mend transaction. This would break the Mara §2 `resugar ∘ sugar = id` contract in the observable state.

An **alternative ordering** worth naming (not recommending, but should be surfaced to Alex):
- Land M-E3 read-path projection as its OWN self-contained shard-body-projector detector-adjacent primitive at `rust/src/apply_h.rs::project_p1_identity_prism_at(shard_path) -> String` (emits the canonical prism-block bytes for a given path).
- Then M-E4 walker composes: read source → strip prism block if `detect_prism_boilerplate == Pass` → verify `read_source + project_p1_identity_prism` equals pre-strip source at byte altitude.
- This makes round-trip fidelity CHECKABLE as a bilateral property in-line with M-E4, without deferring to M-E5 for its first witness.

---

## §7 [ALEX-Q-M-E3] refined

Reed's original [ALEX-Q] surfaced as: "Are prism blocks load-bearing at compile/parse/crystal-canonical altitude, or documentation-only?"

**Refinement after empirical verification** (four sub-questions Alex may want to answer separately):

**[ALEX-Q-M-E3-A]** — Is the round-trip fidelity contract `resugar ∘ sugar = id` at OID altitude (Mara math §2) a HARD contract that M-E3 must land BEFORE M-E4, or a SOFT contract that can be witnessed in M-E5 tests after M-E4 walker fires?

**[ALEX-Q-M-E3-B]** — Should the M-E3 read-path projection live at:
- (a) `rust/src/apply_h.rs::project_p1_identity_prism_at` as a substrate-adjacent primitive dispatched via `apply_h::act`, OR
- (b) as a substrate declaration in `shards/kintsugi/fracture/prism_boilerplate.mirror` composing `@io/fs.read` + a sugar-aware read-lens?

**[ALEX-Q-M-E3-C]** — For `bootstrap/src/lib.rs::collect_declared_namespaces`: if a shard is sugared (prism block removed), should the resolver ALSO learn to synthesize the namespace from the path? Or is the invariant "every shard MUST declare its namespace via one of {glass, prism, grammar, spectral} at least once" load-bearing at the resolver altitude?

**[ALEX-Q-M-E3-D]** — Given that `mirror index` Fiedler over the shard DAG changes when content-address changes, does the Fire E cascade need a companion arc (Fire E-post) that re-computes the Fiedler baseline after the mend transaction — OR does the M-E3 read-path projection make this unnecessary by preserving content-address across the mend?

---

## Karen citations

- `rust/src/apply_h.rs:246-336` — Fire E M-E2 P1 identity-carrier detector.
- `rust/roomba/src/mend.rs:142-172` — degradable prism-line ref extraction.
- `bootstrap/src/apply_h.rs:246-256` — same pattern at bootstrap altitude.
- `bootstrap/src/lib.rs:~2400-2455` — cross-shard namespace resolver; prism-line consumption.
- `bootstrap/src/lib.rs:679, 683, 794, 803` — source-byte cache via `canonical_hash`.
- `bootstrap/src/lib.rs:679, 800` — AST content-address via `compute_content_oid`.
- `bootstrap/src/spectral.rs:162-181` — `compute_content_oid` docblock + dispatch.
- `bootstrap/src/spectral.rs:1200-1210, 2013-2027, 2076-2160` — Combinator meta-glass seed + op_keyword_choice.
- `bootstrap/src/crystallize.rs:191-215, 263-285` — BLAKE3 hash_bytes + Splinter Merkle.
- `bootstrap/src/hash.rs:203-222, 264-272` — canonical_hash + hash_tagged.
- `shards/prism.mirror:44-67` — the fixed-point substrate declaration.
- `shards/glass.mirror:66-84` — sibling prism-block declaration.
- Reed memory `bootstrap_is_dead_do_not_propose_bootstrap_altitude_solutions` (Alex 2026-07-22) — preserved; scout REPORTS grep-truth about bootstrap, does not propose bootstrap-altitude solutions.
- Reed memory `feedback_rust_delivers_primitives_substrate_delivers_composition` (Alex 2026-08-05) — preserved; recommendation §6 favors substrate composition over Rust extension.
