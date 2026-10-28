# mirror's fixpoint parser — spec

*2026-10-28. Mara. Status: **Red** (nothing in this spec runs yet; §13 is the red-test order).*

**Math root:** `docs/math/2026-10-28-mara-mirror-fixpoint-parser-math-foundation.md` (cited below as "math §n"). This spec says what gets built and how it is tested. Why each piece is true lives there and is not re-derived here.

**Decisions this spec builds on (Alex, 2026-10-28, in-transcript):**
- the evaluator lives in mirror ("Otherwise it's decorative");
- no serde: mirror does the transformation itself;
- the parser is built in mirror from prism's settled shape; `bootstrap/` and `boot/` are inspiration at most;
- the first shard is `mirror.mirror` → `@mirror`, with the Splinter as the atom of the AST;
- `mirror compile` = parse, β-normalize, crystallize, back-project into git;
- the Splinter name collision is by design ("We rotate them");
- the ruler is not free; `@ruler` is the mechanism (Reed, relaying Alex), and it lands after this parser.

---

## §1 What `mirror compile <file>` does

One pipeline, each stage one function, each stage's output content-addressed:

| # | stage | input → output | math |
|---|---|---|---|
| 1 | **read** | bytes → concrete tree `T` (every byte in exactly one token or trivia leaf), computed by the engine `E` over the grammar normal form for the file's format | §5.1 |
| 2 | **strip** | `T` → AST `A` (trivia leaves and spans removed) | §3.1 |
| 3 | **normalize** | `A` → `N` (de Bruijn for bound variables; R1–R4 to normal form; SCC members sorted by rule name; named references never unfolded) | §3.4, §1.4 |
| 4 | **settle** | `(T, N)` → `(N, ε)`, with `ε` the erasure record | §3.3 |
| 5 | **crystallize** | `N` → Merkle DAG of splinters, hash-consed, root `addr(N)`; also `addr(ε)` and the source lens | §1.2 |
| 6 | **back-project** | DAG → native git objects (blobs and trees); refs written | §7.3 |
| 7 | **graph** | DAG → `Γ(N)` (undirected, OID-keyed, multiplicity-preserving) | §7.4, Def. 7.6 |
| 8 | **identify** | `addr(N)` and `Γ(N)` → `uuid_spectral` | §7.4 |

Today (`rust/src/compile.rs`, `rust/spectral/src/liquid.rs`) `compile` does none of stages 1–4. `compile_from_source` calls `extract_properties` / `extract_spec_properties` (line scans for property declarations), evaluates them and chains the verdicts. On a pact shard this prints "SAGA chain of 0 crystal(s)". This spec replaces the front of that pipeline and keeps the verdict chain as what runs *after* stage 5.

### §1.1 Default output: one mirror-text record

Per `docs/specs/kintsugi-ci-v0.1.md` §1.4 (mirror-text is the default; JSON only at the `@io` boundary):

```text
verdict   success
file      "mirror.mirror"
root      <addr(N), 64 hex, BLAKE3>
source    <addr of the source lens, 64 hex>
erasure   <addr(ε), 64 hex>
git       <git object id of g(root): 40 hex (sha1 repo) or 64 hex (sha256 repo)>
uuid      <uuid_spectral, 8-4-4-4-12>
crystals  <number of distinct splinters in the DAG>
holes     <number of `\` in N>
undefined <number of undefined (e, i) atoms; 0 for a well-formed grammar>
```

`verdict` is `success` (no holes, no undefined atoms), `partial` (holes or undefined atoms, all named in the record) or `dark` (the input could not be read at all). Exit codes follow `docs/specs/strict-and-total-classification.md`.

`--out @data/json` prints the same fields as JSON through the JSON grammar's printer (§9). That is an `@io` crossing.

### §1.2 Four identifiers, never one column

`root`, `source`, `git` and `uuid` are four different values (math §7.1). No code path compares one with another. `git` is computed from the canonical bytes by git's rule (`"<type> <len>\0" ‖ payload`) and is never derived from `root`'s string.

---

## §2 What `mirror.mirror` must contain

This spec does not write `mirror.mirror`. It states what the file must contain for the fixed point to be checkable.

1. **Header.** `grammar @mirror { … }`, and `out @mirror`.
2. **Every form `mirror.mirror` itself uses** (the subset `L₀`): `grammar`, `in`, `out`, `type` (records and `|` sums), the five op keywords, `io` / abstract action signatures, `pact` / `requires`, `settle … by:`, comments, `\`, `@`-references, string and number literals. If the file uses a form, the file declares it. That's the self-description condition (math §6.2).
3. **PEG semantics.** Choice is ordered, and alternative order is part of meaning (math §2.4). Repetition is greedy, and lookahead (`!e`, `&e`) consumes nothing.
4. **Well-formed.** No direct or mutual left recursion, and no repetition over an expression that can succeed without consuming input (Ford's `WF`, math §5.1). Checked statically (§13, T6).
5. **Rule names are output labels.** A rule's name labels the AST fragment it produces. Renaming a rule is a change of meaning (math §1.4, §2.5).
6. **The meaning declaration `μ`, in exactly three places** (math §2.2):
   - which rules produce **trivia** (layout always; comments per the decision in §12, item 2);
   - which positions **bind** variables (pact variables `?x`, `select |v|`, action parameters): these are α-renamed;
   - which **rewrite rules** the normalizer applies (R1–R4 of `docs/math/2026-08-10-mara-beta-normal-ast-content-addressing-math-foundation.md` §1.2, with the R5 exemption).
7. **`split` as syntax.** The verb exists in the grammar (math §4.2–4.3: choice needs it). Its semantics on the spectral triple live in prismqueer.
8. **Ruler positions.** `settle`'s `by:` and every threshold or bin position accept an `@ruler` reference (§10). Until `@ruler` lands, a bare number in a ruler position reads as `Opaque` ("whose ruler?"), mirror-005's rule.
9. **Holes.** `\` is an atom, valid wherever a body or value is expected, and never reduced (math §3.4).

---

## §3 The bootstrap stages

| stage | what | identity | status after stage 2 |
|---|---|---|---|
| 0 | seed reader `P₀` for exactly `L₀` | `addr(P₀)` (its source shard, if mirror) or the binary's hash | witness only |
| 1 | `P₁ = E(N₀)`: the engine run on `mirror.mirror`'s normal form | the pair `(addr(E), addr(N₀))` | the parser |
| 2 | `P₁` reads `mirror.mirror` again; `addr(N₁) == addr(N₀)` | — | the fixed-point test |

- **Where the seed lives (DECISION, Seam gate).** A hand-written reader for `L₀` is capability, and the FROZEN rule (AGENTS.md, "Boundary Rust") says capability isn't Rust. Two admissible shapes: (a) the seed is a mirror shard executed by the existing floor; (b) the seed is boundary Rust carrying `[substrate-floor:@io-boundary]` with Seam's audit, on the argument that it is the irreducible floor that runs `E`. The math names this floor the irreducible witness (math §6.1). The spec doesn't pick: it requires the choice to be made before the first green commit, in an audit file.
- **Second witness (DDC).** `boot/std/mirror/grammar.mirror` read through `E` (or an independent seed in another language) must produce the same `addr(N₀)` on the `L₀` subset both cover. That is Wheeler's diverse double-compiling, applied to the seed (math §6.2). It raises the cost of a self-consistent misreading without excluding one.
- **Iteration budget.** Stage 2 is required at `k = 1`. Beyond that, `mirror compile --fixpoint` may iterate `N_{k+1} = F(N_k)` up to a declared `k_max` (a ruler; `halts.mirror` clause b) and reports `fixed` / `cycle(p)` / `dark` (math §6.3).

---

## §4 The engine `E`: one rotation core for parsing and inference

- **Core.** Semi-naive evaluation of positive Datalog over a finite domain (math §5.2): a tick is a round, only the delta joins, and it stops at the first empty delta. Atoms are splinters (altitude `@parse/item` for `m(e,i,j)` / `f(e,i)`; `@pact/fact` for pact facts). Each newly derived atom is **deposited** with its tick as the deposit's coordinate. The tick is never part of the atom's address (math Prop. 1.7).
- **Index.** `m` is indexed by `(e, i)`, and Ford's functionality (at most one `j`) is asserted. With the index, a sequence rule joins in O(1), which is what makes the parse linear (math §5.4, CONJECTURE until T6b measures it).
- **PEG.** The grammar normal form compiles to the Horn clauses of math §5.1. No negation: failure is derived positively. An `(e, i)` with neither `m` nor `f` at the fixed point is **undefined**. It is reported as a `\` with the grammar location, never as a crash or a hang.
- **Pacts.** `mirror pact eval` (`rust/src/pact.rs`, `1324aae`) becomes the well-founded wrapper around the same core: two core calls per round (`gamma` with the negative context fixed), `under` ascending and `over` descending (math §5.3). Retractions on the `over` side are deposited as status splinters. The store is append-only. The oracle stays `tests/pact/oracles.md` and the vectors.
- **Not in the core.** Kuramoto phases, coupling and the Fiedler gauge of `prismqueer::spectral::rotation` (math §5.4). The engine shares the deposit-delta-compare shape with `rotation.rs`, not its dynamics.
- **One store.** Parse items, AST splinters, pact facts and rotation observations are all splinters under one address function (§5) and one git projection (§7). Altitude tags keep them apart (math Cor. 1.3).

---

## §5 Addressing

- **Hash.** `H` = BLAKE3-256 over the canonical encoding (math §7.2). Not `prismqueer::Oid::hash`: its digest runs through three `f64` sums (math Prop. 7.2). Until Alex and prism-Reed decide otherwise, `addr` and `Oid::hash` are different functions, and no code may substitute one for the other.
- **Encodings** (math §1.2): `atom = H("splinter\0" ‖ τ ‖ enc(content))`; `fragment = H("fragment\0" ‖ τ ‖ enc(label) ‖ k ‖ addr(x₁) ‖ … ‖ addr(x_k))` with **ordered** children; `lens = H("lens\0" ‖ …)`. `enc` is length-prefixed. `τ` is the altitude `@`-ref in UTF-8, then NUL.
- **Not in the address:** transparency / luminosity, source spans, ticks, witnesses (actor, time, place), the erasure record.
- **Cycles.** Rule references inside a strongly connected component are `(SCC address, index)`. The SCC is one fragment whose children are its rules sorted by name (math §1.4).
- **Sorted children are not allowed here.** prismqueer's `MerkleTree` convention ("sorted by Oid") is the store-merge composition, not syntax (math §1.3).
- **Holes.** A `\` is addressed by the question it asks: `hole(signature)` (math §3.4; DECISION §12, item 4).

---

## §6 The erasure record

`settle` writes `ε` at altitude `@mirror/erasure`: one entry per erased item, `(kind ∈ {layout, comment, span, binder, redex}, path, payload)` (math §3.2–3.3). `ε` is a fragment with its own address, and the source lens binds `addr(N)` and `addr(ε)`.

Requirement: `render(N, ε)` reproduces the input bytes exactly (math Prop. 3.2). The kintsugi formatter is `print(N)` with `ε` ignored, and it must satisfy `compile(print(N)) = N` (math Prop. 3.3).

---

## §7 Back-projection into git

Layout (math Def. 7.3):
- atom → blob (payload = canonical encoding, including `τ`);
- fragment → tree: entry `@` (blob: `τ ‖ label`), then `000`, `001`, … (zero-padded to the width of the largest index in that tree), modes `100644` / `040000`;
- lens → tree: `@`, then `^000 …` for the targets;
- objects are written into the repository's own object database (`git hash-object -w`, `git mktree`), in the repository's object format (SHA-1 or SHA-256).

Refs (so `git gc` keeps what the graph keeps):
- `refs/mirror/root/<addr(N)>` → git id of `g(N)`;
- `refs/mirror/source/<source addr>` → git id of the source lens tree;
- the `addr` ↔ git-id map is recomputable in both directions (math Prop. 7.4), so it is not stored separately.

This writes native git objects. The namespaced `@mirror/store/git` wire (`.git/mirror/`) is not used by this pipeline (DECISION §12, item 6). Nothing is written outside `.git/`. AGENTS.md: "Git IS the content store."

---

## §8 `uuid_spectral` from compile

- `dark` = `addr(N)[0..80)`. Identity comparisons read `dark` only.
- `active` = quantization of a spectral reading of `Γ(N)` (math Def. 7.6: undirected, OID-keyed, **multiplicity-preserving**; reuse is `b₁`, Lemma 7.7). The **graph builder is a stage of its own** (stage 7), because no landed builder keeps reuse (Taut's scout: `coincidence::Laplacian::from_tree` allocates per occurrence; prism's `Graph` collapses parallel edges).
- The operator (undirected Laplacian `L₀ + L₁` of `Γ` is the lean; the directed adjacency is useless on a DAG, math Lemma 7.8), the five axes, the bin edges and the 48-bit split are **one `@ruler`, fixed system-wide and signed by every issuer**. The uuid has no room to name its ruler (math §7.4).
- **Until that ruler is signed, `active = 0`**, as every production caller does today (Taut §1, Stage 6). `compile` prints the uuid with `active = 0` and says so in the record (`active unsigned`). It never fills active bits from an unsigned quantizer.
- `active` never identifies: cospectral non-isomorphic graphs exist, and among ASTs (trees) collisions are generic (math Prop. 7.11).

---

## §9 Wire formats: one engine, no serde

- A format is `(G_fmt, μ_fmt)`: a grammar plus its meaning declaration (math §8). `read_fmt = settle ∘ nf ∘ strip ∘ E(G_fmt)`; `emit_fmt = print_fmt`.
- Formats in scope for this arc: `mirror` (`mirror.mirror`), `mirror-text` (records, kintsugi-ci §1.4), `json` (RFC 8259).
- `μ_json` declares: member order free (sorted in normal form); whitespace trivia; duplicate names → `\`; numbers as exact decimals (never `f64`); escapes free; Unicode normalization (NFC) a ruler.
- Shifting a value into JSON is partial (no `\`, no `@`-refs). What can't be carried is logged in `ε`, and the output marks the gap.
- **No serde on the compile path.** The `json` reader and printer are `E(G_json)` and `print_json`. `pact.rs`'s `serde_json` input parser is replaced by `E(G_pact-json)` with the same vectors as oracle.

---

## §10 Rulers: `@ruler` is the mechanism

`@ruler` is its own shard. It lands **after** this parser, because it needs the grammar (Reed, relaying Alex). This spec names it as the only mechanism for every ruler below and invents no other.

The parser's contract with `@ruler`:
- **ruler positions** (`settle`'s `by:`, thresholds, bins, budgets) accept only `@ruler` references; a bare number there is `Opaque` ("whose ruler?");
- **free positions** (`focus`, `shift`, `project`, `split`) refuse a ruler (math §4.4: only `settle` is non-injective);
- **dark never includes a ruler.** The result of a ruler-carrying op carries the ruler's address in its own content, so the same meaning settled by two rulers gives two results, each saying which ruler made it;
- instrument resolution (machine epsilon) is declared; the multiplier on top of it is signed.

Rulers to declare once `@ruler` exists (math §9): the commutator thresholds `1e-12`, `1e-6`; the Refract cut-off `1e-12`; settle's `Noise`; Kuramoto `K`, `dt`, `omega_scale`; rotation tick budgets; the stage-iteration budget `k_max`; the `active` quantizer, operator and bit split (system-wide); prism `Precision`; JSON's Unicode normalization.

---

## §11 What this spec does not do

- It doesn't write `mirror.mirror`, `@ruler` or any Rust.
- It doesn't pick `SpectralCoordinate<5>`'s definition (three conflict; math §7.4) and doesn't sign the `active` ruler.
- It doesn't change `prismqueer::Oid::hash` or `rotation::Splinter`. It states what the math needs from them (§12) and leaves the change to prism-Reed.
- It doesn't use λ-β on λ-terms. The normalizer for this stage is Definition 3.4 of the math. Adding λ-β needs a typing discipline first (math §3.4).

---

## §12 Decisions needed before green

| # | decision | owner | Mara-lean | math |
|---|---|---|---|---|
| 1 | fragment = composite splinter (supersedes 2026-07-22's "fragment = `@eigen(Crystal<T>)`") | Alex | yes | §1.1 |
| 2 | comments: trivia for the semantic root, `@nl` splinters linked by lens, inside the source address | Alex | yes | §3.3 |
| 3 | rule names are meaning; declaration order isn't | Alex | yes | §1.4 |
| 4 | holes addressed by signature (same question, same address) | Alex | yes, Seam to attack | §3.4 |
| 5 | `H` = BLAKE3 over canonical bytes; `Oid::hash` stays a measurement, not the identity | Alex + prism-Reed | yes | §7.2 |
| 6 | native git objects supersede the `.git/mirror/` namespaced wire | Alex | yes | §7.3 |
| 7 | `rotation::Splinter`: move `tick` out of the address into a deposit record; add `altitude`, `transparency` | Alex + prism-Reed | yes | §1.5 |
| 8 | `split` as a Fracture node in prism's trace DAG (prism CURRENT §5), or carried by mirror's engine until then | prism-Reed | either | §4.3 |
| 9 | where the stage-0 seed lives (mirror shard vs. audited boundary Rust) | Alex + Seam | mirror shard if the floor can run it | §6.1 |
| 10 | `SpectralCoordinate<5>`: one definition of three; `L₀ + L₁` vs `L₀`; the 48-bit split | Alex | fragmentation's Laplacian definition with `L₀ + L₁` | §7.4 |
| 11 | the `active` `@ruler`: system-wide, signed by every issuer | Alex + every issuer | — | §7.4 |
| 12 | prismqueer `lib.rs` docstring "project — the lossy transformation" vs. §7b″ "project is emission" | prism-Reed | fix the docstring | §3.3 |

---

## §13 Red tests, in order

Each test is written red first, against the binary (`mirror compile …`), the way `rust/tests/pact_vectors.rs` drives `mirror pact eval`. Fixtures live under `tests/fixpoint/`.

**T1. The fixed point (the first test; it gates everything else).**
`mirror compile mirror.mirror`:
1. exits 0 and prints a record with `verdict success`, a non-empty crystal DAG (`crystals > 0`) and a `root`;
2. a second run prints the same `root`;
3. `tests/fixpoint/mirror.reformatted.mirror` (same file with layout changed and declarations reordered; generated by a fixture script, checked in) prints the same `root`;
4. the stage-1 parser (`mirror compile --stage 1 mirror.mirror`, i.e. `E(N₀)` reading the file) prints the same `root` (stage 2);
5. `git cat-file -t <git>` prints `tree`, and walking that tree (`@` entry plus numbered entries, recursively) reproduces the fragments: the walk's re-encoding hashes to `root`.

**T2. Meaning changes change the root.**
- one literal in one rule changed → different `root`, `git` and `dark`;
- two alternatives of one ordered choice swapped → different `root` (the PEG shear; math Prop. 2.4);
- one rule renamed → different `root` (rule names are meaning).

**T3. Free moves don't.** Layout, declaration order, α-renaming a pact variable, and (per decision 2) a comment edit: same `root`, same `dark`. A comment edit changes `source`.

**T4. Bytes round trip.** `mirror kintsugi --render mirror.mirror` (render `N` with `ε`) reproduces the file byte for byte, for the original and for the reformatted copy.

**T5. Meaning round trip.** `print(N)` compiled again gives `N`, and printing is idempotent (`print(compile(print(N))) == print(N)`).

**T6. Well-formedness and undefined atoms.**
- a) `mirror.mirror` passes the static `WF` check;
- b) a fixture with direct left recursion (`A ← A "a" / "a"`) compiles to `verdict partial` with `undefined > 0` naming `A`, within a time bound; no hang, no crash;
- c) parse time grows linearly on `mirror.mirror` concatenated `k` times, `k ∈ {1, 2, 4, 8}` (measures the CONJECTURE in math §5.4).

**T7. Holes.** A fixture with two `\` bodies of different signatures: `holes 2`, both survive in `N`, with two distinct addresses. Two `\` with the same signature share one address (decision 4).

**T8. Normalizer confluence.** Enumerate the critical pairs of R1–R4 over `shards/` and check every pair joins (math Prop. 3.7). Any non-joining pair is reported as a `\` against the rule set.

**T9. The graph builder.** Fixture `f(x, x)` with a shared `x`: `Γ` has one `x` vertex and two parallel edges, and `b₁ = 1`. A fixture with `r` reusing `c` at depth 1 and depth 2 has `b₁` equal to its reuse count (math Lemma 7.7). The per-occurrence expansion of the same fixture has `b₁ = 0`. That case is the negative control.

**T10. `uuid_spectral`.**
- `uuid` of `mirror.mirror`: `dark` is stable across two compiles and across the reformatted copy, and changes when one rule's meaning changes (T2's first edit);
- until the `active` `@ruler` is signed: `active == 0`, and the record says `active unsigned`;
- after it is signed: `active` is stable across two compiles and the reformatted copy;
- two cospectral, non-isomorphic fixtures get the same `active` and different `dark`, and identity comparison reports them as different.

**T11. Diverse double reading.** The second witness (§3) produces the same `addr(N₀)` on the `L₀` subset both cover.

**T12. JSON through the engine (mirror-001's tests).** Two JSON events differing only in key order and whitespace → same `root`; changing one field value → different `root`; a duplicate key → `verdict partial` with a `\`. No `serde` symbol is linked into the compile path: `nm` on the release binary, filtered to the compile module, shows no `serde_json`.

**T13. One engine.** `mirror pact eval` vectors 01, 03 and 04 pass through `E`'s core, and semi-naive equals naive on every vector (math Prop. 5.4). The over-side retraction of math Prop. 5.5 appears as a deposited status splinter, not a deletion.

**T14. Rulers (after `@ruler` lands).** `settle … by:` with a bare number is `Opaque` ("whose ruler?"); `settle` with no `by:` is a compile error; an `@ruler` reference in a `focus` or `shift` position is a compile error.

**T15. One Splinter (prism side; forwarded to prism-Reed).** Two rotation ticks that observe the same state deposit **one** splinter twice, i.e. one address and two deposit records (math Prop. 1.7).

---

## §14 Pickup

The order for the builder: T1 red, the seed (decision 9), `E`'s core with the PEG clauses, `mirror.mirror`, then T1 green. T2–T7 next, because they pin meaning. T8–T13 after that. T14 waits for `@ruler`; T15 belongs to prism.
