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

> **Amended 2026-10-28 after Seam's crowbar** (`docs/audits/2026-10-28-seam-crowbar-mirror-fixpoint-parser.md`, `358f430` `86ae959` `ce06cbf`). Every change is marked *[Seam]* where it lands. The math is amended to match (math §12 has the verdict table and the three points where I disagree).

> **Amended 2026-10-29 (Alex, ratified: "yes … excitiiiing!"): `mirror compile` compiles a single shard; `mirror kintsugi` does kintsugi.** Compile computes **local sections**: one shard in, one crystal out. It reads that shard's bytes and its format grammar, nothing else, and it can only report **holes** ("I don't know here"). Gluing compiled shards, and reporting **commas** ("you two disagree"), is `mirror kintsugi`: `docs/specs/mirror-kintsugi.md`. Math: `docs/math/2026-10-29-mara-compile-kintsugi-local-sections-and-gluing.md`. Every change is marked *[10-29]*. The same pass pins the byte encoding and settles six ambiguities the T1 builder found (`9dbcb13`), marked *[T1]*.

> **Amended again 2026-10-29 after Seam's crowbar** (`docs/audits/2026-10-29-seam-crowbar-compile-kintsugi.md`) **and Alex's seven defaults** (relayed by Reed; adopted as defaults, revisable once the compiler reports real cases). Marked *[Seam n]* (his §12 fold item) and *[A n]* (Alex's default n, listed in `mirror-kintsugi.md` §10).

## §1 What `mirror compile <file>` does

*[10-29]* **One shard.** `compile` reads the bytes of one file and the grammar of its format (`@mirror` by default; §9), and nothing else: no other shard, no `in @X` target, **no path** (decision 13: identity by content, not path). *[Seam 8; A2]* Its inputs are exactly **`(addr(format grammar), start, bytes)`**: the start rule is a caller-named input (§1.2′), and the path is not an input at all, because a shard with no `out` gets a `name?` hole instead of a name from its path (§2 item 6). The cache key is that triple, and the result is addressed by `root`. What the shard needs from other shards is recorded in its **interface** (§1.3), and checked by `mirror kintsugi`.

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
| 9 | **interface** *[10-29]* | `N` → the interface fragment `I(N)` (§1.3), addressed | 10-29 math §1 |

Today (`rust/src/compile.rs`, `rust/spectral/src/liquid.rs`) `compile` does none of stages 1–4. `compile_from_source` calls `extract_properties` / `extract_spec_properties` (line scans for property declarations), evaluates them and chains the verdicts. On a pact shard this prints "SAGA chain of 0 crystal(s)". This spec replaces the front of that pipeline and keeps the verdict chain as what runs *after* stage 5.

### §1.1 Default output: one mirror-text record

Per `docs/specs/kintsugi-ci-v0.1.md` §1.4 (mirror-text is the default; JSON only at the `@io` boundary):

```text
verdict    success
file       "mirror.mirror"
start      <the start rule the parse used, e.g. @mirror.shard>
root       <addr(N), 64 hex, BLAKE3>
source     <addr of the source lens, 64 hex>
erasure    <addr(ε), 64 hex>
interface  <addr(I(N)), 64 hex: the shard's stalk, §1.3>
git        <git object id of g(root): 40 hex (sha1 repo) or 64 hex (sha256 repo)>
uuid       <uuid_spectral, 32 hex, no hyphens (not an RFC 9562 UUID; DECISION §12) >
active     unsigned
crystals   <number of distinct splinters reachable from root: fragments + atoms, §7>
holes      <number of `\` occurrences in N>
undefined  <number of undefined (e, i) atoms in the start atom's dependency cone>
undeclared <number of code-line references to @X with no `in @X`, §1.3>
wf         <true | false: Ford's well-formedness of the grammar, reported separately>
```

*[T1]* `active` is `unsigned` until the `active` `@ruler` is signed (§8); after that it is `signed <addr of the @ruler>`. It is a field of the record, not a comment on `uuid`.

*[10-29]* After the summary record, one record per finding, blank-line separated, in normal-form path order:

```text
hole       <question address, 64 hex>
at         <path in N>
line       <line:col from ε; a source coordinate, for people, never identity>

undeclared @tool/git
at         <path in N of the first referring code line>
line       <line:col from ε>

free       verdict
at         <path in N>
```

`free` records list bare names the shard uses and doesn't declare (§1.3). Compile can't bind them, so they are not findings at compile. Kintsugi binds them.

`verdict` is the oriented ternary plus dark:
- `success`: no holes, no undefined atoms, no Dark lines;
- `partial`: holes, undefined atoms or Dark lines, all named. *[Alex 10-29, second round]* An **undeclared reference is a Dark line**: it points into what compile can't see (nothing looked). A hole is Opaque (a question asked). Success lines plus a Dark line compose to `partial`;
- `fail`: a pact or property the shard states about itself evaluates false (the verdict chain that runs after stage 5). *[T1 (b)]* **`fail` is the record word**, in compile and in kintsugi alike (the family success / partial / fail / dark, as in `kintsugi-ci-v0.1.md`). "Red" is the traffic-light *reading* of `fail` and of `mirror love`'s third state, never a value printed in a `verdict` field;
- `dark`: the input could not be read at all.

**Exit codes** *[A5; Seam 10]*: one table for compile, kintsugi and `mirror love` (which already maps Green → 0, Yellow → 1, Red → 255, `rust/src/main.rs`, the block after `fn cmd_love`):

| verdict | exit |
|---|---|
| `success` | 0 |
| `partial` | 1 |
| `fail` | 255 |
| `dark` | 2 |

- Partial, red and dark are three different codes, so CI can tell an honest hole from a disagreement.
- **Tolerance is a declared ruler, not a flag.** What a gate accepts (lean: partial yes; red and dark no) is an `@ruler` declared in `mirror.spec` (`mirror-kintsugi.md` §4). `--strict` is retired: it existed to make partial non-zero, which it now is.
- T1.1's "exits 0" asks for `verdict success` and is consistent with this table.
- Docs that assume partial = 0 and change with this table: `docs/specs/kintsugi-ci-v0.1.md:119` ("0 on `partial` (unless `fail-on: partial`)"; `fail-on` becomes the gate ruler), `docs/specs/compiler-surface-plan.md` (~914, "non-strict: Partial → exit 0"), `docs/specs/strict-and-total-classification.md` (step 5: dark keeps exit 2; `--strict` retired). Those files get their own edit with this pass's pickup (§14).
- The landed `rust/src/compile.rs` exits 0 on `fail`; this table replaces it.

`--out @data/json` prints the same fields as JSON through the JSON grammar's printer (§9). That is an `@io` crossing.

### §1.2 Four identifiers, never one column

`root`, `source`, `git` and `uuid` are four different values (math §7.1). No code path compares one with another. `git` is computed from the canonical bytes by git's rule (`"<type> <len>\0" ‖ payload`) and is never derived from `root`'s string. *[10-29]* `interface` is a fifth: the address of `I(N)`, a different fragment from `N`. Two shards with equal `interface` and different `root` are expected (a body changed, the contract didn't).

### §1.2′ Which parser: `--stage` *[T1]*

- `mirror compile --stage 0 <file>` reads with the seed `P₀`.
- `mirror compile --stage 1 <file>` reads with `P₁ = E(N₀)`, where `N₀` is the normal form of `mirror.mirror` as read by `P₀` (cached by `root`; recomputed when `mirror.mirror`'s bytes change).
- **Plain `mirror compile <file>` is `--stage 1`.** It is the parser the substrate trusts once the engine exists. Before `E` exists, plain compile is red: it does not silently fall back to the seed.
- On any file, `mirror.mirror` included, the parser reads the file under its format grammar (`@mirror` unless `--in <format>` names another, §9) and prints one §1.1 record. Nothing is special about `mirror.mirror` except that it *is* the `@mirror` grammar.
- **Start** *[Seam 8, 9; Alex 10-29 second round]*: `--start <@grammar.rule>` names it. Without it, the format's **default start** is used, pinned per format by this spec: for `@mirror`, the exported rule `shard` (`mirror.mirror` must export a rule of that name); for `@data/json`, `value`; for `@data/mirror-text`, `records`. The record's `start` field says which was used. A format with no pinned default and no `--start` reads as `partial` with the hole "start?".
- *[T1 (f)]* **Every option takes its value in either form**, `--stage 0` or `--stage=0` (likewise `--start`, `--in`, `--out`). The two spellings parse to the same option and print the same record. An option given twice is an error, never last-wins.
- Consequence for T1.4: comparing plain against `--stage 1` compares a thing with itself. T1.4 compares **`--stage 0` against `--stage 1`** (amended below).

### §1.3 The interface, and the one lint compile owns *[10-29]*

**Interface `I(N)`** (the shard's stalk; math §1). A fragment at altitude `@mirror/interface`, computed from `N` alone:

| part | content | addressed by |
|---|---|---|
| `name` | the shard's `out @X`, or the hole `name?` when it has none *[A2]* | the name |
| `in` | the `in @X` lines, sorted | the names |
| `export` | each exported declaration: `(name, kind, arity, addr)` with `kind ∈ {carrier, action, pact, bilateral, prism, grammar, rule, keyword}` *[Seam 9]* (`rule`: a grammar rule, a start candidate; `keyword`: an exported token, sigil or operator such as `out in`, `out @`, `out #`); `addr` = address of the declaration's normal form. For an action: its signature **and** its `requires` clauses, not its body | Merkle |
| `uses` | *[Seam 8]* per **qualified** `@X`: each name the code lines use as `@X.name`, with its **use kind** (`carrier` in a type position; `action/k` applied to `k` arguments; `pact` in `requires`) | names and kinds |
| `free` | *[Seam 8]* the **bare** names the code lines use and the shard doesn't declare (`ref`, `verdict`, …), with use kinds. Compile can't tell which in-target binds them; kintsugi does | names and kinds |
| `undeclared` | *[Seam 8]* the `@X` with code-line references and no `in @X`. Part of the interface, so `interface` changes when one appears, and kintsugi can key it as a pseudo-line | names |
| `requires` | the `requires` clauses the shard states about imported names (pacts on another shard's behaviour) | addresses |

Carriers and actions are different kinds, and the restriction to a line keeps the kind (the June insight §4.3(b): "a B that imports A's carriers is a different section from a B that imports A's actions").

**The lint compile owns.** With `references(S, X)` = some **code line** of `S` (not a comment, not inside a string literal, not an `in` / `out` / `source` line) mentions `@X` or `@X.name`, where `X` is the whole `/`-path before any `.`:

```text
undeclared(S, X) :- references(S, X), not imports(S, X), not declares(S, X).
```

- `declares(S, X)` reads the `out @X` / `grammar @X` / `prism @X` in `N`. Nothing is inserted from the path *[A2]*, so a shard without `out` that names itself is `undeclared` on itself and has a `name?` hole: one finding per cause.
- **Comments are what `μ_mirror` says they are** *[Seam 12]*: the comment tokens come from the grammar, never from a hard-coded list in the lint or its oracle. At `346253b` the corpus uses `#`, `//` (2 files) and `--` (32 files) as line comments; the oracle `tests/kintsugi/oracles/golden_k0.py` uses exactly these three, and `mirror.mirror` must declare the same set or the fixture changes with it.
- **Unit: `(S, X)` pairs** *[Seam 12]*. The `undeclared` field counts distinct pairs, not occurrences and not shards. At `346253b` under the three comment tokens: **44 pairs in 33 shards** (`golden_k0.shards.out`).
- **Exact match.** `in @kintsugi` does not cover `@kintsugi/roomba` (DECISION §12 row 21, lean exact): `in` imports *a shard* (Alex 2026-07-23), and a prefix rule would let a change to `@kintsugi/roomba` escape every line that should re-check it (math Prop. 4.2, stalk completeness).
- **Prose is reported, not failed.** References inside comments are counted in a separate field only with `--prose` (`prose_refs <n>`), never in `verdict`. Counting prose would force cycles (`@sheaf` → `@psychohistory` → `@sheaf`, Taut 2026-10-29 §2.3).
- This lint is `@kintsugi/fracture/inport` (`shards/kintsugi/fracture/inport.mirror`, Alex's name: "inport = `in`-port"). The substrate already had the word; compile is where its `detect` runs, because it is decidable from one shard's bytes (that shard's own §5 proof).
- The converse rule, an `in @X` with no shard `X`, needs the shard set and belongs to kintsugi (a **gap**, `mirror-kintsugi.md` §3).


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
   - which **rewrite rules** the normalizer applies (R1, R2, R4 of `docs/math/2026-08-10-mara-beta-normal-ast-content-addressing-math-foundation.md` §1.2). ~~*[Seam]* R3 is **reversed**: it inserts `out τ_path(path)` when absent and keeps it when present, never elides.~~ *[A2, 10-29; Seam 8]* **R3 is removed**: it neither elides nor inserts. A shard without `out` compiles to `partial` with the hole `name?`, so compile never reads the path (inserting from the path made the path a hidden input, Seam's §7.1). The R5 exemption tests **content** (what the shard declares), never the path. Path-dependent elision gives identical bytes different roots and different exports equal roots (math Prop. 3.8).
   - only **decidable** free moves (math §2.2 note).
   - *[Seam]* `mirror.mirror`'s own name `@mirror` is the one identity root that can't carry its own address. It is a ruler signed by Alex. Every other format grammar is referenced as `(name, addr(grammar))`.
7. **`split` as syntax.** The verb exists in the grammar (math §4.2–4.3: choice needs it). Its semantics on the spectral triple live in prismqueer.
8. **Ruler positions.** `settle`'s `by:` and every threshold or bin position accept an `@ruler` reference (§10). Until `@ruler` lands, a bare number in a ruler position reads as `Opaque` ("whose ruler?"), mirror-005's rule.
9. **Holes.** `\` is an atom, valid wherever a body or value is expected, and never reduced (math §3.4). Each hole has a **question** and an **occurrence** (§5.1).
9′. *[2026-10-29, for `mirror pact eval`; Seam's pact-eval audit §1.2]* **The pact clause form.** `mirror.mirror` declares:
    - Datalog rules `atom requires literal, …` with `?x` binders;
    - **`not`** as a literal prefix inside `requires`;
    - **both conjunction spellings**, comma-separated after one `requires` and one `requires` per line. They normalize to one form: a free move (§2.3 of the math), since both denote the same conjunction.

    This makes the hand reader's clause layer a sublanguage of `G_pact`, so its fixtures stay valid at retirement.
10. *[T1; Alex 10-29 second round; Seam 9]* **The start rule is named, never positional, and it is what the grammar exports.** No new keyword: a grammar's entry points are its exported rules (`out <rule>`), and the caller names one (`@mirror.shard`). *[T1 (a)]* **A rule's qualified name is `<grammar name>.<rule>`**: `grammar @mirror { … }` (item 1) gives `@mirror.shard`, the same `@X.name` form as any reference (§1.3). `@mirror/grammar` is the legacy `boot/std/mirror/grammar.mirror` grammar, a different name (slated to become `@mirror/prism`), and never the qualifier of `mirror.mirror`'s rules. The closure derives every rule at every position at once, so the start is the question an observer puts to it. **Start candidates = exported names whose declaration is a rule** (kind `rule`); an exported keyword or sigil (`out in`, `out @`, `out #` in `boot/`) is kind `keyword` and never a start. Order stays out of meaning, so T1.3 holds. The start is an input to compile (§1), not part of the grammar's `root`.

---

## §3 The bootstrap stages

| stage | what | identity | status after stage 2 |
|---|---|---|---|
| 0 | seed reader `P₀` for exactly `L₀` | `addr(P₀)` (its source shard, if mirror) or the binary's hash | witness only |
| 1 | `P₁ = E(N₀)`: the engine run on `mirror.mirror`'s normal form | the pair `(addr(E), addr(N₀))` | the parser |
| 2 | `P₁` reads `mirror.mirror` again; `addr(N₁) == addr(N₀)` | — | the fixed-point test |

- **Where the seed lives (DECISION, Seam gate).** A hand-written reader for `L₀` is capability, and the FROZEN rule (AGENTS.md, "Boundary Rust") says capability isn't Rust. Two admissible shapes: (a) the seed is a mirror shard executed by the existing floor; (b) the seed is boundary Rust carrying `[substrate-floor:@io-boundary]` with Seam's audit, on the argument that it is the irreducible floor that runs `E`. The math names this floor the irreducible witness (math §6.1). The spec doesn't pick: it requires the choice to be made before the first green commit, in an audit file.
- **The trust root is `E`, not the seed** *[Seam; math Prop. 6.4]*. Stage 2 checks the seed against `E`. It can't see `E` misreading constructs `mirror.mirror` doesn't exercise on itself. Seam built two engines that pass stage 2 and a seed-DDC while misreading user grammars: `E_long` (ordered choice read as longest match) and `E_troj` (`!` read as `&` when a rule `Admin` exists). Three requirements follow:
  - **DDC on `E`'s source:** build `E` from its shard with two independent floors and require bit-identical `E` artefacts (Wheeler);
  - **a semantics oracle independent of `mirror.mirror`:** `tests/fixpoint/semantics/`, user grammars and inputs with known verdicts (Prop. 2.4's shear, `!`/`&` vectors, Ford's examples, Seam's `E_long`/`E_troj` vectors);
  - the **seed witness** (`boot/std/mirror/grammar.mirror` through `E`, or a seed in another language) stays as a cheaper check on `addr(N₀)`.
- **In nix terms** *[handoff mirror-007, 2026-10-29]*: the floor that runs `E` (`packages.mirror`: rustc, prism at the `Cargo.lock` rev, the nixpkgs pin) is the one declared trusted input, like nixpkgs' `bootstrap-tools`; `checks.fixpoint` checks stage 2 against it, and T11a's DDC (two independent floors, bit-identical `E`) is its own check. Nix reproduces the floor; it doesn't justify it, and the trust root stays `E`, not the seed.
- **Iteration.** Stage 2 is required at `k = 1`. Reaching a fixed point only at `k > 1` is a **failed** stage 2, because one extra step would launder a seed misreading *[Seam]*. `mirror compile --fixpoint` may iterate as a diagnostic up to a declared `k_max` (a ruler; `halts.mirror` clause b). It stops on **consecutive** equality and reports `fixed at k` / `cycle(p)` / `dark` (math Prop. 1.9, §6.3).

---

## §4 The engine `E`: one rotation core for parsing and inference

- **Core.** Semi-naive evaluation of positive Datalog over a finite domain (math §5.2): a tick is a round, only the delta joins, and it stops at the first empty delta. Atoms are splinters (altitude `@parse/item` for `m(e,i,j)` / `f(e,i)`; `@pact/fact` for pact facts). Each newly derived atom is **deposited** with its tick as the deposit's coordinate. The tick is never part of the atom's address (math Prop. 1.7).
- **Index.** `m` is indexed by `(e, i)`, and Ford's functionality (at most one `j`) is asserted. With the index, a sequence rule joins in O(1), which is what makes the parse linear (math §5.4, CONJECTURE until T6b measures it).
- **PEG.** The grammar normal form compiles to the Horn clauses of math §5.1. No negation: failure is derived positively. An `(e, i)` with neither `m` nor `f` at the fixed point is **undefined**. It is reported as a `\` with the grammar location, never as a crash or a hang.
- **Pacts.** `mirror pact eval` (`rust/src/pact.rs`, `1324aae`) becomes the well-founded wrapper around the same core: two core calls per round (`gamma` with the negative context fixed), `under` ascending and `over` descending (math §5.3). Retractions on the `over` side are deposited as status splinters. The store is append-only. The oracle stays `tests/pact/oracles.md` and the vectors.
- **Not in the core.** Kuramoto phases, coupling and the Fiedler gauge of `prismqueer::spectral::rotation` (math §5.4). The engine shares the deposit-delta-compare shape with `rotation.rs`, not its dynamics.
- **Two stop tests, never mixed** *[Seam; math Prop. 1.9]*. Monotone steps (the Datalog core) stop on an empty delta, i.e. the cumulative set is unchanged. Non-monotone steps (rotation, the stage iteration) stop on consecutive equality of the state's address. A recurrence at a distance `p > 1` is `cycle(p)`, never fixed.
- **Undefined atoms counted in the cone** *[Seam]*. `undefined` counts only atoms reached by the start atom's first-premise evaluation (math §5.1). `S <- 'a' / L ; L <- L 'b'` on `a` is `success` with `wf false`.
- **One store.** Parse items, AST splinters, pact facts and rotation observations are all splinters under one address function (§5) and one git projection (§7). Altitude tags keep them apart (math Cor. 1.3).

### §4.1 The contract `E` meets before any parser exists *[10-29]*

The smallest input/output contract under which `E` is testable with no grammar, no `mirror.mirror` and no parser. Where `E` lives (shard or boundary Rust) doesn't change it. Math: §5.1–§5.3.

**Terms.** Function-free (Datalog). A term is a **variable** or a **constant**, and a constant is one of three tagged kinds: `sym` (a UTF-8 name), `nat` (an unsigned integer; positions) or `addr` (32 bytes; a splinter). There is **no arithmetic and no comparison in the core**. What the PEG clauses need arrives as extensional facts generated with the input ("EDB"):
- `next(i, j)` for `j = i + 1`;
- `lt(i, j)` for `i < j`, over `0..n`;
- `lit(t, i, j)` when the input's bytes `[i, j)` equal terminal `t`;
- `nolit(t, i)` when they don't (math §5.1's `¬input`, the stratum-0 negation, precomputed).

The core sees no negation at all.

**Program.** A finite list of rules `id: h :- b₁, …, b_k` (k ≥ 0) over atoms `p(t₁, …, t_n)`, with each predicate at a fixed arity.
- **Safe**: every head variable occurs in a body atom.
- An unsafe rule or an arity mismatch makes the program `dark` (not run). *[review 10-29]* A function term can't be written (`Term` has no such case), so there is nothing to check. §4.2 moves both checks to construction.
- Optional declarations `functional p(1, 2) → 3` assert at most one value of argument 3 per value of arguments (1, 2). That is Ford's functionality for `m(e, i, j)`.
  - **Positions are 1-based**, lie in `1..=arity`, and the value position is outside the key.
  - `E` uses the key index on the **`M` side of a join only**; the `Δ` side is scanned. Linear parse time stays a CONJECTURE (math §5.4) until T6c measures it.
  - **Several values for one key** give one finding per extra value, each paired with the least value in `Const` order: values `v₁ < v₂ < v₃` give `(v₁, v₂)` and `(v₁, v₃)`.

**Fact store.** A set of ground atoms at **one altitude per store** *[review]*. The facts and every atom derived from them in that run share `τ` (`@parse/item`, `@pact/fact`, or `@datalog/fact` for tests). Bytes, pinned *[review A1, A2]*:
- **one formula, §5.0's**: `addr = H("splinter\0" ‖ τ ‖ enc(content))` with `content = enc(p) ‖ u64(n) ‖ arg₁ ‖ … ‖ arg_n`. The earlier §4.1 text inlined `content` without its outer `enc`; §5.0 wins, so every atom in the system has one shape. The builder's `addr` gains the outer length prefix: a one-line change of derived hex, which no test pins yet (Seam's engine audit §3.2);
- each `arg` = tag byte ‖ `enc(bytes)`, with tags **`sym 0x00`, `nat 0x01`, `addr 0x02`**. `sym` bytes are UTF-8. `nat` bytes are `u64` big-endian (so `enc` = 8 ‖ 8 bytes). `addr` bytes are the 32 raw bytes;
- **engine altitudes use the root-grammar flag**: `τ = UTF-8(altitude) ‖ 0x00 ‖ 0x00`. They are the engine's own vocabulary, not a format grammar's. A grammar an atom is about enters through its content: `m(e, i, j)` carries `e` as an `addr` constant, and `e`'s address carries its grammar. Domain separation (math Cor. 1.3) holds because the altitude names differ;
- `M`'s address: `H("fragment\0" ‖ τ ‖ enc("model") ‖ u64(|M|) ‖ addr₁ ‖ …)`, children sorted by address;
- a **status splinter** (W's retraction) is an atom at `@pact/status` with content `enc("status") ‖ u64(3) ‖ (addr, the retracted atom) ‖ (sym, "undefined") ‖ (sym, "false")`. Its **round is a deposit coordinate, never in its address** (math Prop. 1.7), like a tick;
- two atoms are the same atom iff their addresses are equal;
- the store is append-only.

**A run.** `E(P, F, k_max) → (M, D, stop)`:
- `M`: the least model, `F ⊆ M`. Its address is the store-merge fragment of `M` (children sorted by address, which is right here because `M` is a set, math §1.3).
- `D`: one **deposit** per atom of `M`, `(addr, tick)`, where tick 0 holds the facts and tick t the atoms first derived in round t. Deposits are records beside the atoms. The tick is never in an atom's address or in `M`'s address (math Prop. 1.7).
- `stop`: `fixed at T` (the first round whose delta is empty; the last deposit tick is `T − 1`), or `budget at k_max` (a non-empty delta at the budget; `k_max ≥ 1`).
- *[review A3; PROVED]* **`T ≤ |B| − |F| + 1 ≤ |B| + 1`.** Every round before `T` adds at least one atom of `B ∖ F`, and round `T` adds none. The earlier bound `k_max ≥ |B|` was off by one: the program `a.` has `|B| = 1`, `F = ∅`, and stops `fixed at 2`. With `k_max ≥ |B| − |F| + 1`, `budget` cannot happen, so the run's stop is `fixed` and there is no "bound exceeded" case to report (§4.2 removes it).
- Functionality violations are returned as findings `functional_violation(p, key, v, v′)`, never as a crash.

**The well-founded wrapper `W`** (math §5.3). Rules may carry negative body literals `not a`.
- `γ(S)` = `M` of one core run of the program with each `not a` read as the EDB test `a ∉ S`.
- `under₀ = F`; `over_k = γ(under_k)`; `under_{k+1} = γ(over_k)`.
- Stop at consecutive equality of `under`, which is monotone, so this is also an empty delta.
- `W` returns `true = under`, `undefined = over ∖ under`, and `outcome` (`clear` iff `undefined = ∅`, else `opaque`). *[review A5]* There is **no query**. The status of any ground atom is read with `status(atom)`: true, undefined, or false (closed world for facts only). The pact oracle's `absent` is `status = false` for an atom no rule heads.
- Each retraction (an atom of `over_k` missing from `over_{k+1}`) is deposited as a **status splinter** (bytes above), with `k + 1` as its round. Nothing is deleted.
- *[review A5]* **`γ` always runs to completion.** Each core call inside `W` gets the sufficient budget `|B| − |F| + 1`, so `γ` can't stop at a budget, and "`γ` hit its budget" is not an outcome. `W`'s own outer loop is bounded by `|B|` rounds (`under` grows each round).
- *[review A5]* **Functionality findings under `W`** are computed on `true` (definite violations). Undefined atoms can't violate definitively, and the `γ` calls' intermediate findings are not reported.
- *[review; PROVED, §4.3]* At the end, `γ(under) = over` and `γ(over) = under`, so every undefined atom flips at each step of `γ` from `under`: **undefined = period 2**.

**Red tests, in order** (`rust/tests/engine_*.rs` or the engine shard's own harness). Fixtures are **data in the test** or mirror-text files. No JSON is read and no serde is linked.

- **E1. Semi-naive = naive** (math Prop. 5.4). Equal `M` and equal tick per atom on each fixture (a deposit's tick is the first naive round containing the atom):
  - transitive closure over pact vector 01's graph (the cycle `b → c → b`);
  - a chain of length 10;
  - 200 random positive programs from a fixed seed, generated in the test.
- **E2. Empty-delta stop.** Facts `e(0,1) … e(n−1,n)` with `p(X,Y) :- e(X,Y). p(X,Z) :- p(X,Y), e(Y,Z).` give `stop fixed at n + 1`, the last deposit at tick `n`, and `|M| = n + n(n+1)/2`. A program with no rules gives `M = F` and `stop fixed at 1`.
- **E3. The tick is not in the address.** One atom derived at tick 1 in one run and at tick 3 in another has one address. Every permutation of `P` and `F` gives the same `M` address and the same deposit ticks.
- **E4. The PEG clauses of math §5.1 on hand-built grammars.** Clauses are written as data, with the EDB above. Ford's functionality holds in every case.
  - `S <- 'a' S / 'a'` on `aaa` → `m(S,0,3)`.
  - Seam's `E_long` vector `Top <- S !. ; S <- 'a' / 'ab'` on `ab` → `f(Top,0)` (ordered choice, not longest match).
  - The `E_troj` vector (a rule `Admin` present; `Guard <- !'root' Name`) on `root` → `f(Guard,0)`.
  - `S <- 'a' / L ; L <- L 'b'` on `a` → `m(S,0,1)`, and `(L,0)` has neither `m` nor `f`.
  - `A <- A 'a' / 'a'` on `aa` → `(A,0)` undefined, `stop fixed` (no hang).
  - `S <- &'a' 'a'`: on `a` → `m(S,0,1)`; on `b` → `f(S,0)`.
  - The undefined-in-cone count belongs to the parse wrapper, not to `E`.
- **E5. Pacts through `W`.** Expected results are transcribed from `tests/pact/oracles.md` and vectors 01, 03 and 04 (branch `reed/mirror-006-pact`, `5babfbad`):
  - 01: the nine `path` facts true, `clear`;
  - 03: `send(a1)` false, `clear`;
  - 04: `true {r}`, `undefined {p, q, t}`, `opaque`.
  - Math Prop. 5.5's example (`b :- not a. c :- not b. d :- not c.`, fact `a`) deposits one status splinter for `d` at round 1 and deletes nothing.
- **E6. Order never matters.** For every vector marked `permute`, the result record is byte-identical under the runner's fixed sample of permutations of rules and facts.
- **E7. No serde.** The engine's crate (or shard) has no `serde` dependency: `cargo tree -i serde` on it is empty.

### §4.2 The type-level contract (Alex: "No bare types. Make illegal state unrepresentable.") *[review 10-29]*

A review of `rust/engine/src/lib.rs` at `83fe665` against §4.1. For each runtime check or unreachable variant, the type that removes it. ~~The builder lands this as ♻️: no behaviour change~~ *[engine builder, 2026-10-29]* **§4.2 is not behaviour-preserving, so it lands as a 🔴/🟢 pair, not a ♻️.** E1–E7 and the existing tests stay green except where a decision below changes an observable outcome. The address bytes of §4.1 A1 still land as their own 🟢 first. Decisions (Reed, 2026-10-29):
- **Negation in the core** moves from a runtime `Dark::Negation` to a `compile_fail` doctest on `PositiveRule` (its body is `Vec<Atom>`, so a negative literal can't be written).
- **`Position::new(0)` is `None`**: the constructor checks it.
- **`Dark::Functional { pred, position }`** names the **first offending position in declaration order**: one outside `1..=arity`, or the value position when it sits inside the key.
- **Arity mismatches** are found by `Instance::new` and reported with `expected` = the program's arity and `found` = the fact's arity.

*[builder findings at slice 2, `1ae650de`; recorded 2026-10-29]*
- **`Instance::new` is generic over a sealed `Program` trait** (`PositiveProgram`, `NormalProgram`). That adds one public name, `Program`. Accepted: the trait is sealed, so no outside implementation can exist.
- **Arity conflicts among rules alone** (no facts involved) report the **least two** arities seen for the predicate (`expected` = least, `found` = next), so the report doesn't depend on rule order.
- **`Run` fields are still `pub`**, so a test or caller could build a `Run<Fixed>` that no run produced, forging the typestate's guarantee. **Decision (spec-owned, Mara):** the fields become private, with read-only accessors, and `Run` is constructible only inside the engine. It lands as a 🔴/🟢 pair: a `compile_fail` doctest building `Run { … }` outside the crate. Tests that need a `Run` get one by running.

**Newtypes (no bare `u64` / `usize` / `String` / `[u8; 32]` in the public API).**

| type | wraps | invariant it carries | removes |
|---|---|---|---|
| `Addr` | `[u8; 32]` | a BLAKE3 output; only `addr()`, `fragment()` and a checked parse construct it | `pub type Addr = [u8; 32]`, which lets any 32 bytes pose as an address |
| `Altitude` | `String` | starts with `@`, contains no NUL, non-empty path segments | the silent NUL collision in `put_tau`; a misspelt altitude producing valid hashes |
| `Sym`, `Predicate`, `Var`, `RuleId` | `String` | UTF-8, no NUL; four distinct types | passing a variable name where a predicate is expected (`Atom.pred: String`, `Term::Var(String)`, `Rule.id: String`) |
| `Nat` | `u64` | a domain constant | `Const::Nat(u64)` mixing with counts |
| `Arity` | `u32` | the arity a predicate is declared at | `usize` arities in `Dark::Arity` |
| `Position` | `NonZeroU32` | 1-based, `≤ arity`, checked against the predicate's `Arity` at construction | `Functional.key: Vec<usize>` plus the 1-based→0-based subtraction in `keys()` (an underflow if 0 slips through) |
| `Tick` | `u64` | a core round; 0 = facts | `Deposit.tick: u64`, `Stop::Fixed(u64)` |
| `Round` | `u64` | a `W` outer round; distinct from `Tick` | `Retraction.round: u64` being comparable with a tick |
| `Budget` | `NonZeroU64` | `k_max ≥ 1` | `k_max = 0` (A4) |

**Typestates.**
- **`PositiveProgram` and `NormalProgram`.** Built only by `PositiveProgram::new(rules, decls) -> Result<PositiveProgram, Dark>` (and likewise `NormalProgram::new`). Construction does what `validate()` does at run time today: arity, range restriction (safety), declaration positions. A `PositiveRule` has `body: Vec<Atom>`, so **negation in the core can't be written**. `Dark::Negation` disappears, and `run` takes `&PositiveProgram` and can't fail on safety.
- **`Instance<'p>`** = a program plus a `Facts` store whose arities agree with the program's signature, built by `Instance::new(&program, facts) -> Result<_, Dark>`. That is where the facts-vs-rules arity check happens. After it, `run` is total.
- **`Bounds`.** `Instance::sufficient_budget() -> Budget` returns `|B| − |F| + 1` (PROVED, §4.1).
  - `run_to_fixpoint(&Instance) -> Run<Fixed>` uses that budget. Its `stop` field is `Fixed(Tick)` with no `Budget` case.
  - `run_bounded(&Instance, Budget) -> Run<Bounded>` keeps both cases.
  - `W` calls `run_to_fixpoint`, so **`Dark::Budget` disappears**.
  - **`Dark::Bound` is removed.** It is unreachable by Prop. 5.4, so it becomes a `debug_assert!`, never an API variant.
- **What remains of `Dark`** is only what construction finds: `Unsafe { rule: RuleId, var: Var }`, `Arity { pred: Predicate, expected: Arity, found: Arity }`, `Functional { pred: Predicate, position: Position }`. A `Run` never carries a `Dark`.
- **`FunctionalViolation { pred, key: Vec<Const>, least: Const, other: Const }`.** Field names say the pairing rule (§4.1), and `least < other` is checked at construction.
- **`WellFounded::status(&self, &Fact) -> Status`** stays (no query, A5). The altitude is the `Instance`'s, so `status` can't be asked about an atom at another altitude.

**Which runtime checks go away, in one line each:** arity (construction); safety (construction); negation in the core (unwritable); declaration positions (`Position` checked against `Arity`); `k_max = 0` (`NonZeroU64`); `γ`'s budget (`Run<Fixed>`); the Prop. 5.4 bound (`debug_assert!`); NUL in an altitude (`Altitude::new`); 0-based/1-based confusion (`Position`).

### §4.3 `Frequency`: one shape for every iterating thing *[review 10-29]*

Math: `docs/math/2026-10-29-mara-engine-frequency-and-kleinos.md` §1. Every deterministic iteration in the system (E's rounds, W's alternation, `rotation::Splinter` (decision 7), `compile --fixpoint` (§3), convergence passes, kintsugi re-runs) ends one of three ways: **fixed** (period 1, `X = COORD(X)`), **cycle(p)** (period p ≥ 2), or **unknown within a window** (the budget). One trait gives all of them that shape:

```rust
pub trait Frequency {
    type State;
    type Kind: OrbitKind;                       // Monotone | General
    fn step(&self, s: &Self::State) -> Self::State;
    fn addr(s: &Self::State) -> Addr;           // equality of states = equality of addresses
}

pub enum Orbit<K: OrbitKind> {
    Fixed { at: Tick },                         // λ = 1, first reached at tick μ
    Cycle { from: Tick, period: Period, _k: K::CycleWitness },  // λ ≥ 2
    Unknown { window: Budget },                 // μ + λ > window: a fact about the observer
}
// Period: NonZeroU64 with value ≥ 2 (a period of 1 is Fixed, never Cycle).
// Monotone::CycleWitness = core::convert::Infallible: Cycle is unconstructible
// for inflationary steps (law F4), so E's stop type has two cases, as now.
```

**Laws** (each PROVED in the math doc §1):
- **F1 (rho):** on a finite state set, every orbit has a unique tail length `μ ≥ 0` and period `λ ≥ 1` with `s_{t+λ} = s_t` for all `t ≥ μ`.
- **F2 (divisibility):** for `t ≥ μ`, `s_t = s_{t+q}` iff `λ | q`. Every later recurrence is a multiple of the period.
- **F3:** fixed ⇔ `λ = 1` ⇔ `step(s_μ) = s_μ`.
- **F4 (monotone ⇒ no cycle):** if `s ⊆ step(s)` on a finite powerset, `λ = 1`. E's core is this case.
- **F5 (phase from deposits):** with every tick's address deposited, the first repeated address, at tick `t` with earlier tick `t′`, gives exactly `μ = t′` and `λ = t − t′`. For `t ≥ μ` the phase is `(t − μ) mod λ`. Without the deposits, Brent's or Floyd's constant-memory cycle finding gives the same `(μ, λ)` in `O(μ + λ)` steps.
- **F6 (W):** `W`'s outer sequence `under_k` is monotone (`λ = 1`). The raw antitone step `γ` from `under` has `λ ≤ 2`, and the atoms with period 2 are exactly the undefined ones. The comma inside one program is a frequency.

**Consecutive equality only detects `λ = 1`.** That is why compile §3 and decision 7 report `cycle(p)` separately. A non-monotone loop that only compares consecutive states calls a 2-cycle "not converged" forever and reaches its budget. F5 is the remedy, and it costs only the deposits the store already keeps.

**Spectral side: honest for linear steps, analogy for the rest** (math §1.4):
- for `x ↦ Ax`, an orbit of period `λ` lives in `ker(A^λ − I)`, where `A`'s eigenvalues are `λ`-th roots of unity, and `λ` is the lcm of their orders (PROVED);
- for any finite step map, the 0/1 matrix of its functional graph has spectrum = all `p`-th roots of unity for each cycle of length `p`, plus `0` for the tails (PROVED). This is exact but lives on the state space (`2^|B|` states for E), so it is a statement, not a computation;
- Kuramoto rotation is non-linear, and its address equality passes through a quantizing ruler, so "frequency" there is an analogy until a linearisation is declared.

### §4.4 Kleinos at the engine layer: red tests *[review 10-29]*

Math: the frequency/kleinos doc §2. Notation: `lfp(P, F)` = `M` of `run_to_fixpoint`, `E₃ = lfp(P₁ ∪ P₂, F₁ ∪ F₂) ∖ (lfp(P₁, F₁) ∪ lfp(P₂, F₂))`, as a set of addresses.

- **K-E1 (sovereignty).** On 500 seeded pairs of positive programs, `lfp(Pᵢ, Fᵢ) ⊆ lfp(P₁ ∪ P₂, F₁ ∪ F₂)`.
- **K-E2 (the emergent third, as addresses).**
  - `P₁ = {q(X) :- p(X)}`, `F₁ = {p(a)}`; `P₂ = {r(X) :- q(X)}`, `F₂ = ∅` → `E₃ = {addr(r(a))}`.
  - The same with `P₂ = ∅`, `F₂ = {p(a)}` (they cross on `p`, nothing new) → `E₃ = ∅`.
  - `P₁ = {path(X,Y) :- e(X,Y)}` with `F₁` = edges `0→1, 1→2, 2→3`; `P₂ = {path(X,Z) :- path(X,Y), path(Y,Z)}` with `F₂` = edges `3→4, 4→5, 5→0` → `E₃` = all 36 `path` atoms of the 6-cycle minus the 3 that `P₁` derives alone, so `|E₃| = 33 > 1`. That refutes "exactly one emergent third" at this layer.
- **K-E3 (crossing is necessary).** On the seeded pairs, `E₃ ≠ ∅` implies a crossing edge in the predicate dependency graph (a body predicate of one side that the other side heads or holds facts of).
- **K-E4 (fusion is idempotence).** `P ∪ P = P`. Composing a program with itself gives the same model address and `E₃ = ∅`.
- **K-E5 (no Fiedler law; math only).** In K-E2's first fixture, `λ₂` of the dependency graph falls from 2 (each part is `K₂`) to 1 (the union is `P₃`) while `E₃ ≠ ∅`. Recorded as a counterexample, not a test.
- **K-E6 (retraction under W: a part disagrees with the glue).** `P₁ = {p :- not q}`, `P₂ = {q.}`: `W(P₁)` has `p` true, `W(P₁ ∪ P₂)` has `p` false and `q` true. The record lists `{p}` as the part/glue disagreement. On positive pairs that set is always empty (K-E1).
- **K-E7 (the tick's shape).** For every deposit at tick `t ≥ 1` on the E1 fixtures, some rule instance derives it with every premise at tick `≤ t − 1` and at least one premise at exactly `t − 1`.
- **F-E1 (frequency).**
  - Vector 04: `γ(under) = over` and `γ(over) = under`, and the period-2 atoms are `{p, q, t}`.
  - A hand-built general step of period 3 with a tail of 2: deposits give `μ = 2`, `λ = 3`.
  - A budget below `μ + λ` gives `Unknown`.
---

## §5 Addressing

- **Hash.** `H` = BLAKE3-256 over the canonical encoding (math §7.2). Not `prismqueer::Oid::hash`: its digest runs through three `f64` sums (math Prop. 7.2). Until Alex and prism-Reed decide otherwise, `addr` and `Oid::hash` are different functions, and no code may substitute one for the other.
- **Encodings** (math §1.2): `atom = H("splinter\0" ‖ τ ‖ enc(content))`; `fragment = H("fragment\0" ‖ τ ‖ enc(label) ‖ k ‖ addr(x₁) ‖ … ‖ addr(x_k))` with **ordered** children; `lens = H("lens\0" ‖ …)`. `enc` is length-prefixed. `τ` is the altitude `@`-ref in UTF-8, then NUL. *[Seam]* For any altitude defined by a format grammar other than `@mirror` itself, `τ` also carries `addr(format grammar)`, so a change of `μ` changes every address computed under it.
- **Not in the address:** transparency / luminosity, source spans, ticks, witnesses (actor, time, place), the erasure record.
- **Cycles.** Rule references inside a strongly connected component are `(SCC address, index)`. The SCC is one fragment whose children are its rules sorted by name (math §1.4).
- **Sorted children are not allowed here.** prismqueer's `MerkleTree` convention ("sorted by Oid") is the store-merge composition, not syntax (math §1.3).
- **Holes.** See §5.1. ~~A `\` is addressed by `hole(signature)`.~~ *[Seam: FAILS. Two different functions with the same signature hash-cons to one hole, and every untyped value hole shares one address.]*

### §5.0 Bytes, pinned *[T1]*

The encoding of §5 at byte level. Spec-owned (DECISION §12 row 19): any injective, prefix-free choice satisfies math Prop. 1.1, and this one is fixed so that two implementations agree bit for bit.

| symbol | bytes |
|---|---|
| `u64(n)` | 8 bytes, unsigned, big-endian |
| `enc(b)` | `u64(len(b)) ‖ b` |
| `τ` | UTF-8 of the altitude `@`-ref, then `0x00`, then `0x00` for the root grammar `@mirror`, or `0x01 ‖` the 32 raw bytes of `addr(format grammar)` for every other format (decision 14). A name never contains NUL, so `τ` is prefix-free |
| `k` | `u64(k)` |
| child address | 32 raw bytes (never hex) |
| `H` | BLAKE3, default (unkeyed) mode, 32-byte output |

- atom: `addr = H("splinter\0" ‖ τ ‖ enc(content))`
- fragment: `addr = H("fragment\0" ‖ τ ‖ enc(ℓ) ‖ u64(k) ‖ addr(x₀) ‖ … ‖ addr(x_{k−1}))`
- lens: `addr = H("lens\0" ‖ τ ‖ enc(ℓ) ‖ u64(k) ‖ addr(t₀) ‖ …)`
- **The kind prefix (`splinter\0`, `fragment\0`, `lens\0`) is never stored.** In git it is carried by the object's shape (§7): a blob is an atom; a tree with numbered entries is a fragment; a tree with `^` entries is a lens. Everything after the kind prefix and before the child addresses is stored verbatim (§7), so the read path re-hashes without re-deriving anything.

### §5.1 Holes and resolutions: rerere for commas

This is what the arc is for: mirror is git rerere for commas (Alex, 2026-10-28, relayed by Reed). git's rerere records "hand resolve results" and applies them "to their corresponding automerge results". Mirror does the same for `\`, with two identifiers per hole (math Def. 3.9):

| identifier | definition | role |
|---|---|---|
| **question** | `H("hole\0" ‖ τ ‖ enc(Γ ⊢ T) ‖ enc(pacts in scope))`; the context de Bruijn-normalised | resolutions are **offered** by question: same question, same address, so one recorded answer is offered everywhere it is asked |
| **occurrence** | `(addr(N), path in N)`, a deposit coordinate (invariant under free moves, because `N` is the quotient) | resolutions are **applied** by occurrence; `mirror fate <occurrence> <resolution>` |

- The question atom sits in `N`, so two holes asking the same question share it, and `Γ` shows that as reuse. That's intended (math §12, disagreement 2).
- Recording a resolution writes a splinter `resolution(question, body)`. Applying it at an occurrence writes a new crystal; nothing is mutated.
- `holes` in the record counts **occurrences**.

---

## §6 The erasure record

`settle` writes `ε` at altitude `@mirror/erasure`: one entry per erased item, `(kind ∈ {layout, comment, span, binder, redex}, path, payload)` (math §3.2–3.3). `ε` is a fragment with its own address, and the source lens binds `addr(N)` and `addr(ε)`.

Requirement: `render(N, ε)` reproduces the input bytes exactly (math Prop. 3.2). The kintsugi formatter is `print(N)` with `ε` ignored, and it must satisfy `compile(print(N)) = N` (math Prop. 3.3).

---

## §7 Back-projection into git

Layout (math Def. 7.3; bytes pinned *[T1]* against §5.0):
- atom → blob, payload **`τ ‖ enc(content)`**, so `addr = H("splinter\0" ‖ payload)`;
- fragment → tree: entry `@` (blob, payload **`τ ‖ enc(ℓ)`**), then the children as numbered entries, modes `100644` / `040000`;
- lens → tree: `@` (same payload shape), then `^`-prefixed numbered entries for the targets;
- *[T1 (c)]* **The `τ` in a tree's `@` payload is the fragment's (or lens's) own altitude**: the same `τ` that is in its address preimage (§5.0), never a separate "label altitude". So an `@` payload and an atom blob can be byte-identical (equal `τ`, and `enc(ℓ)` equal to `enc(content)`), and then git stores **one** blob for both. That is git's dedup, and it is harmless: addresses differ by kind prefix (`fragment\0` vs `splinter\0`), and the read path tells them apart by **position** (the `@` entry is always the header; a numbered entry is always a child). A walk never reads an `@` blob as a child atom.
- **child-entry width** *[T1]*: a fixed width per tree, `w = max(3, number of decimal digits of k − 1)`. *[T1 (e)]* For `k = 0` there are no numbered entries, so no width is used; by convention `w(0) = 3`. A fragment with no children (or a lens with no targets) is a tree whose only entry is `@`. So `000 … 009` for ten children and `0000 … 1233` for 1234. Fixed width makes git's byte order of entry names the child order, and the minimum of 3 is the `000` the example always showed. ("Width of the largest index" and "fixed width per tree" now say the same thing.);
- **the read path's re-encoding**: for a tree, `addr = H(kind ‖ @payload ‖ u64(number of numbered entries) ‖ addr(child₀) ‖ …)` with `kind` = `fragment\0` or `lens\0` by entry prefix, children taken in entry order, each child's `addr` computed recursively; for a blob, `addr = H("splinter\0" ‖ blob)`. Every `@` payload starts with `@` (the first byte of `τ`), which is what `9dbcb13`'s T1.5 asserts;
- objects are written into the repository's own object database (`git hash-object -w`, `git mktree`), in the repository's object format (SHA-1 or SHA-256).

Refs (so `git gc` keeps what the graph keeps):
- `refs/mirror/root/<addr(N)>` → git id of `g(N)`;
- `refs/mirror/source/<source addr>` → git id of the source lens tree;
- the `addr` ↔ git-id map is recomputable in both directions (math Prop. 7.4), so it is not stored separately.

**Re-hash on read** *[Seam]*. Every load from git re-walks the tree, re-encodes it and checks it against the `addr` the ref names. A mismatch is dark. A default repository's SHA-1 is not trusted on its own.

This writes native git objects. The namespaced `@mirror/store/git` wire (`.git/mirror/`) is not used by this pipeline (DECISION §12, item 6). Nothing is written outside `.git/`. AGENTS.md: "Git IS the content store."

---

## §8 `uuid_spectral` from compile

- `dark` = `addr(N)[0..80)`. Identity comparisons read `dark` only.
- *[10-29, Alex: "add spectral UUID on the `Fragment`"]* **Every fragment carries a `uuid_spectral`**, not only the root: `dark` from its own `addr`, `active` from the spectrum of its own sub-DAG `Γ(f)` (10-29 math §11). It is derived, cached by `addr`, and stored nowhere that identity reads. Its use is rerere's **similar** offer (`mirror-convergence.md` §4); until the `active` ruler is signed, every fragment's `active` is 0 and only exact offers exist.
- *[Seam]* **Store and merge on `addr` (256 bits), never on the uuid.** An 80-bit truncation doesn't compose: `dark(a ⊔ b)` can't be computed from `dark(a)` and `dark(b)`. The uuid is derived at the edge.
- *[Seam]* `uuid_spectral` sets no RFC 9562 version or variant bits, so it is printed as 32 hex digits, never in the 8-4-4-4-12 form a validator might re-stamp (DECISION §12, item 11).
- *[Seam]* `EMPTY := uuid(compile(empty shard))`, a value compile produces. It replaces `BLAKE3("")[0..10)`.
- `active` = quantization of a spectral reading of `Γ(N)` (math Def. 7.6: undirected, OID-keyed, **multiplicity-preserving**; reuse is `b₁`, Lemma 7.7). The **graph builder is a stage of its own** (stage 7), because no landed builder keeps reuse (Taut's scout: `coincidence::Laplacian::from_tree` allocates per occurrence; prism's `Graph` collapses parallel edges).
- The operator (undirected Laplacian `L₀ + L₁` of `Γ` is the lean; the directed adjacency is useless on a DAG, math Lemma 7.8), the five axes, the bin edges and the 48-bit split are **one `@ruler`, fixed system-wide and signed by every issuer**. The uuid has no room to name its ruler (math §7.4).
- *[Seam]* `b₁` (the reuse count, exact) is an **integer field** of the coordinate, not something read off a float axis. `L₀`'s spectrum already determines it, so `L₀ + L₁` adds nothing (math §7.4 item 1).
- **Until that ruler is signed, `active = 0`**, as every production caller does today (Taut §1, Stage 6). `compile` prints the uuid with `active = 0` and says so in the record's `active` field (`active unsigned`, §1.1 *[T1]*). It never fills active bits from an unsigned quantizer.
- `active` never identifies: cospectral non-isomorphic graphs exist, and among ASTs (trees) collisions are generic (math Prop. 7.11).

---

## §9 Wire formats: one engine, no serde

- A format is `(G_fmt, μ_fmt)`: a grammar plus its meaning declaration (math §8). `read_fmt = settle ∘ nf ∘ strip ∘ E(G_fmt)`; `emit_fmt = print_fmt`.
- Formats in scope for this arc: `mirror` (`mirror.mirror`), `mirror-text` (records, kintsugi-ci §1.4), `json` (RFC 8259).
- `μ_json` declares: member order free (sorted in normal form); whitespace trivia; duplicate names → `\`; numbers as exact decimals (never `f64`); escapes free; Unicode normalization (NFC) a ruler.

### §9.1 Is `1` the same as `1.0`? *[10-29; Reed's bullet-up Response #2]*

spectral's `|> graph` writes weights `1`; Mara's spike wrote `1.0`. Same graph, two ids, unless the normal form decides. The math (10-29 math §7):

- Rewriting `1.0` to `1` is a **free move iff the number type `μ` declares is the exact value**, i.e. the number denotes an element of ℚ (a finite decimal). Then the literals `1`, `1.0`, `1.00`, `1e0`, `10e-1` are representations of one value, and the map literal → value is the quotient the normal form already takes for layout. It is injective on meaning (math §2.3), so it moves nothing.
- It is **not** free if `μ` gives a number more than its value: a declared **precision** (`1.0` asserts a tenth; a measurement reading), or a **type** read off the spelling (`1` an integer, `1.0` a real, in a typed field). Then `1 ≡ 1.0` merges two meanings, which is a `settle` and needs an `@ruler`.
- **`f64` is not the number type**: under IEEE-754 both literals are the same double, but so are decimals that differ in ℚ beyond double precision (e.g. `9007199254740993` and `9007199254740992`); using `f64` would make the identification free for the wrong reason (spec §9: "never `f64`"). RFC 8785 (JCS) canonicalises numbers through IEEE-754 doubles and ECMAScript's serialiser, so its canonical form identifies decimals that differ beyond double precision. It is prior art for the shape, not for the number type.
- **Canonical form if ℚ is chosen:** the shortest decimal that denotes the value: no exponent, no leading `+`, no trailing zeros after the point, no point when the value is an integer, `-0` → `0`. This is a bijection between finite decimals and canonical strings, so Reed's proposal ("integral numbers normalize to integers") is the integer case of it.
- **Default adopted (Alex 10-29, [A6]):** `μ` marks each number position `exact` (data: ℚ, `1 ≡ 1.0`, the spelling goes to the erasure record, so `source` differs and `root` doesn't) or typed (`int`, `f64`, or a ruler: the type is kept and printers read it, so an `f64` field prints as a float and `arity 1.0` at an `int` position is a hole). Unmarked positions are `exact`. The number rule never applies inside an `@`-ref atom (dates and versions in refs are names, Seam §8.3).
- ~~**Until decided: `Opaque`**~~ (superseded by the default above), as Reed proposed: a graph compiled from `1.0` and one from `1` get different roots and the record says why (`number unnormalized`), never a silent coercion.
- **Owner: Alex** (DECISION §12 row 18): which number type `μ_json` and `μ_mirror-text` declare for weights. Mara-lean: exact value (ℚ) for data positions; precision, where it matters, is declared as its own field or as an `@ruler`, never inferred from a trailing zero.
- Shifting a value into JSON is partial (no `\`, no `@`-refs). What can't be carried is logged in `ε`, and the output marks the gap.
- **No serde on the compile path.** The `json` reader and printer are `E(G_json)` and `print_json`. `pact.rs`'s `serde_json` input parser is replaced by `E(G_pact-json)` with the same vectors as oracle.

---

## §10 Rulers: `@ruler` is the mechanism

`@ruler` is its own shard. It lands **after** this parser, because it needs the grammar (Reed, relaying Alex). This spec names it as the only mechanism for every ruler below and invents no other.

The parser's contract with `@ruler`:
- **ruler positions** (`settle`'s `by:`, thresholds, bins, budgets) accept only `@ruler` references; a bare number there is `Opaque` ("whose ruler?");
- **free positions** (`focus`, `shift`, `project`, `split`) refuse a ruler (math §4.4: only `settle` is non-injective);
- **dark never includes an *unnamed* ruler** *[Seam: "never includes a ruler" contradicted the next clause]*. The result of a ruler-carrying op carries the ruler's address in its own content, so the same meaning settled by two rulers gives two results, each saying which ruler made it. PEG alternative order, `μ` (via the grammar address in `τ`) and the root name `@mirror` (signed by Alex) are all named rulers inside dark;
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

**Ratified (Alex 2026-10-28):** Mara's lean on every Alex-owned row below (1, 2, 3, 4, 6, 9, 9′, 13, 14; 5 was already decided). Still open: 7, 8 and 12 (prism-Reed), 10 and 11 (the spectral coordinate and the `active` `@ruler`). On the uuid (11), Alex expects it to become the identity people dial; that is consistent with this spec as long as "same shard?" is answered by the dark half (a truncation of `addr`) and merges are computed on `addr`. The active half is a neighbourhood that moves when its ruler moves, so it never decides identity.

| # | decision | owner | Mara-lean | math |
|---|---|---|---|---|
| 1 | fragment = composite splinter (supersedes 2026-07-22's "fragment = `@eigen(Crystal<T>)`") | Alex | yes | §1.1 |
| 2 | comments: trivia for the semantic root, `@nl` splinters linked by lens, inside the source address | Alex | yes | §3.3 |
| 3 | rule names are meaning; declaration order isn't | Alex | yes | §1.4 |
| 4 | ~~holes addressed by signature~~ **reversed [Seam]:** question address (offer) + occurrence (apply), §5.1 | Alex | yes | Def. 3.9 |
| 5 | `H` = BLAKE3 over canonical bytes; `Oid::hash` stays a measurement, not the identity. **Stands; reason changed [Seam]:** no security argument, float- and `BTreeMap`-order-defined, 17 SHA-256 per byte (not "three scalars") | Alex + prism-Reed | yes | Prop. 7.2′ |
| 6 | native git objects supersede the `.git/mirror/` namespaced wire | Alex | yes | §7.3 |
| 7 | `rotation::Splinter`: move `tick` out of the address into a deposit record; add `altitude`, `transparency`; **[Seam] plus** content quotiented by common phase rotation, and a consecutive-equality stop test with `cycle(p)` | Alex + prism-Reed | yes | Def. 1.8, Prop. 1.9 |
| 8 | **reframed [Seam]:** choice needs a coproduct eliminator. Mirror realises it as `split` + `settle`. **Answered (prism-Reed, prism-004, 2026-10-29): not a Fracture/Join node** (the trace DAG models concurrency, a product: every branch happens). In prism, choice becomes **conflict** in an event structure (Nielsen, Plotkin & Winskel 1981: events are ordered, concurrent, or in conflict): `split` puts the alternatives in conflict, `settle` under a declared ruler keeps one and logs the rest as erasure (the `Tail` shape). A new prism type, red first; until it lands, carried by mirror's engine | prism-Reed | Fracture/Join | Prop. 4.1′ |
| 9 | **demoted [Seam]:** where the seed lives (mirror shard vs. audited boundary Rust). It is no longer the trust root. **Decided (Alex 2026-10-29): (b), boundary Rust** under `[substrate-floor:@io-boundary]`, with Seam's audit file landing before the first green commit (today's `rust/` can't run a shard as a parser) | Alex + Seam | ~~mirror shard if the floor can run it~~ (b) | §6.1 |
| 9′ | **new [Seam]:** DDC on `E`'s source (two floors, bit-identical `E`) plus a semantics oracle independent of `mirror.mirror` | Alex + Seam | yes | Prop. 6.4 |
| 10 | `SpectralCoordinate<5>`: one definition of three; the bit split. ~~`L₀ + L₁` vs `L₀`~~ **void [Seam]:** `L₀` already carries `b₁`, so `b₁` becomes an explicit integer field | Alex | fragmentation's Laplacian definition, with `b₁` exact | §7.4 |
| 11 | the `active` `@ruler`: system-wide, signed by every issuer. **[Seam] plus:** no RFC 9562 form (print 32 hex) or spend 6 bits; `EMPTY` from compile; merge on `addr`, never on the uuid | Alex + every issuer | 80 dark bits, no hyphenated form | §7.4 items 5–7 |
| 13 | **new [Seam]:** reverse R3 (insert, don't elide); R5 by content, not path. **Amended [A2, 10-29]:** R3 removed; no `out` = hole `name?` | Alex | default adopted | Prop. 3.8 |
| 14 | **new [Seam]:** `τ = (name, addr(format grammar))`; the root name `@mirror` is the one signed ruler of identity | Alex | yes | §2.2 note |
| 15 | **new [Seam]:** `undefined` counts only the start atom's dependency cone; `wf` reported separately | — (spec) | yes | §5.1 |
| 16 | *[10-29]* **Ratified (Alex 2026-10-29, "yes … excitiiiing!"):** `mirror compile` compiles one shard (local sections, holes); `mirror kintsugi` glues (commas). `mirror-kintsugi.md` | Alex | — | 10-29 math §0 |
| 17 | *[T1]* the start rule. **Decided (Alex 10-29):** what the grammar exports (`out <rule>`), named by the caller; default per format pinned in §1.2′ | Alex | decided | §2 item 10 |
| 18 | *[10-29]* number type. **Default [A6]:** `μ` declares a number type per position: data positions `exact` (ℚ; `1 ≡ 1.0`; spelling to erasure), typed positions keep their type (an `f64` field prints as a float; `int` positions make `arity 1.0` a hole). Unmarked = `exact` | Alex | default adopted | §9.1; 10-29 math §7 |
| 19 | *[T1]* byte encoding (`u64` big-endian lengths, `τ` layout, stored payloads, child width) | — (spec) | as §5.0, §7 | Prop. 1.1 |
| 20 | *[10-29]* undeclared. **Decided (Alex 10-29):** Dark at the line, `partial` for the document. **Exits [A5]:** success 0, partial 1, red 255, dark 2; gate tolerance is a ruler in `mirror.spec` | Alex | decided / default | §1.1, §1.3 |
| 23 | *[T1 (d)]* `crystals` = the number of distinct splinter addresses reachable from `g(root)`: fragments (trees) and atoms (blobs at numbered entries). `@` blobs are headers, not splinters, and are not counted. Lenses are not reachable from `g(root)` (the source lens points *at* the root), so they are not counted either | — (spec) | as stated; matches `0a7fc73`'s walk | Prop. 7.4 |
| 24 | *[T1]* rows 9 and 17–23 settle the six T1 ambiguities: (a) `@mirror.shard`; (b) `fail` is the record word; (c) `@` payload `τ` = the fragment's own; (d) `crystals` as row 23; (e) `w(0) = 3`, no entries; (f) `--opt v` and `--opt=v` both. Row 9 is decided (b), boundary Rust (`aaa41e3a`). **The commit gate returns with kintsugi v0, not with T1** (`4e8c3d6c`'s Justfile comment): T1 is a gated acceptance test of compile; the gate is `mirror kintsugi mirror.spec` under `ruler @mirror/gate` | — (record) | — | — |
| 22 | *[Seam 8]* compile's cache key `(addr(grammar), start, bytes)`; default start per format pinned (§1.2′; `@mirror` → `shard`) | — (spec) | as §1, §1.2′ | — |
| 21 | *[10-29]* `in @X` covers exactly `@X`, not `@X/…` | Alex | exact (stalk completeness) | 10-29 math Prop. 4.2 |
| 12 | prismqueer `lib.rs` docstring "project — the lossy transformation" vs. §7b″ "project is emission" | prism-Reed | fix the docstring | §3.3 |

---

## §13 Red tests, in order

Each test is written red first, against the binary (`mirror compile …`), the way `rust/tests/pact_vectors.rs` drives `mirror pact eval`. Fixtures live under `tests/fixpoint/`.

**T1. The fixed point (the first test; it gates everything else).** *[Seam]* T1 may not go green before T2's alternative-swap assertion is green: a normalizer that sorted choice children would pass T1 everywhere. And stage 2 must hold at `k = 1`.
`mirror compile mirror.mirror`:
1. exits 0 and prints a record with `verdict success`, a non-empty crystal DAG (`crystals > 0`) and a `root`;
2. a second run prints the same `root`;
3. `tests/fixpoint/mirror.reformatted.mirror` (same file with layout changed and declarations reordered; generated by a fixture script, checked in) prints the same `root`. This holds at a different path because compile never reads the path and R5 is content-based (decision 13 as amended): `mirror.mirror` declares `out @mirror` explicitly;
4. the stage-1 parser (`mirror compile --stage 1 mirror.mirror`, i.e. `E(N₀)` reading the file) prints the same `root` as the seed (`mirror compile --stage 0 mirror.mirror`) (stage 2). *[T1]* Plain `compile` is `--stage 1` (§1.2′), so comparing plain with `--stage 1` would be vacuous;
5. `git cat-file -t <git>` prints `tree`, and walking that tree (`@` entry plus numbered entries, recursively) reproduces the fragments: the walk's re-encoding hashes to `root`.

**T2. Meaning changes change the root.**
- one literal in one rule changed → different `root`, `git` and `dark`;
- two alternatives of one ordered choice swapped → different `root` (the PEG shear; math Prop. 2.4);
- one rule renamed → different `root` (rule names are meaning).

**T3. Free moves don't.** Layout, declaration order, α-renaming a pact variable, and (per decision 2) a comment edit: same `root`, same `dark`. A comment edit changes `source`.

**T4. Bytes round trip.** *[10-29: moved from kintsugi to compile; rendering one shard is local]* `mirror compile --render mirror.mirror` (render `N` with `ε`) reproduces the file byte for byte, for the original and for the reformatted copy.

**T5. Meaning round trip.** `print(N)` compiled again gives `N`, and printing is idempotent (`print(compile(print(N))) == print(N)`).

**T6. Well-formedness and undefined atoms.**
- a) `mirror.mirror` passes the static `WF` check;
- b) a fixture with direct left recursion (`A ← A "a" / "a"`) used by the start rule compiles to `verdict partial` with `undefined > 0` naming `A`, within a time bound; no hang, no crash;
- b′) *[Seam]* `S <- 'a' / L ; L <- L 'b'` on `a`: `verdict success`, `undefined 0` (the `L` atoms are outside the start atom's cone), `wf false`;
- b″) `&e` and `!e` vectors (math §5.1 table, including the `&e` row);
- c) parse time grows linearly on `mirror.mirror` concatenated `k` times, `k ∈ {1, 2, 4, 8}` (measures the CONJECTURE in math §5.4).

**T7. Holes: rerere for commas (§5.1).**
- *[Seam]* `io normalize(x: path) -> path { \ }` and `io parent(x: path) -> path { \ }`: one **question** address, two **occurrences**, `holes 2`;
- the same signature under different `requires`: two question addresses;
- recording a resolution at `normalize`'s occurrence offers it at `parent`'s (same question) but applies it at `parent`'s only on an explicit apply to that occurrence;
- untyped value holes `x = \` in two contexts: question addresses differ whenever `Γ` differs;
- every hole survives normalization.

**T8. Normalizer confluence and path independence.** Enumerate the critical pairs of R1, R2, R4 and reversed R3 over `shards/`, and check every pair joins (math Prop. 3.7). This **includes R1 against R3** (UNTESTED; Seam). Any non-joining pair is reported as a `\` against the rule set. *[Seam]* Path vectors:
- identical bytes at `shards/prism.mirror` and `shards/x/prism.mirror` get the same root;
- ~~a shard without `out` at `shards/a.mirror` and at `shards/b.mirror` gets different roots, because reversed R3 inserts the export.~~ *[A2]* a shard without `out` at `shards/a.mirror` and at `shards/b.mirror` gets the **same** root, `verdict partial`, and one `name?` hole.

**T9. The graph builder.** Fixture `f(x, x)` with a shared `x`: `Γ` has one `x` vertex and two parallel edges, and `b₁ = 1`. A fixture with `r` reusing `c` at depth 1 and depth 2 has `b₁` equal to its reuse count (math Lemma 7.7). The per-occurrence expansion of the same fixture has `b₁ = 0`. That case is the negative control. *[Seam]* `b₁` computed from `L₀` (`tr/2 − n + mult₀`) equals the combinatorial count. A hub fixture (many rules sharing `Var(0)` and one literal) shows how much of `b₁` comes from trivial atoms. That case is informational, not pass/fail.

**T10. `uuid_spectral`.**
- `uuid` of `mirror.mirror`: `dark` is stable across two compiles and across the reformatted copy, and changes when one rule's meaning changes (T2's first edit);
- until the `active` `@ruler` is signed: `active == 0`, and the record says `active unsigned`;
- after it is signed: `active` is stable across two compiles and the reformatted copy;
- two cospectral, non-isomorphic fixtures get the same `active` and different `dark`, and identity comparison reports them as different (*[Seam]* use Laplacian-cospectral trees, which exist at 11 vertices);
- *[Seam]* `uuid` is printed as 32 hex digits; `EMPTY` equals `uuid(compile(empty shard))`; merging two shards is computed on `addr` and the uuid is re-derived, never combined from two uuids.

**T11. Trusting the engine.** *[Seam: the first version checked only the seed.]*
- a) DDC on `E`: `E` built from its shard by two independent floors gives bit-identical artefacts;
- b) the semantics oracle `tests/fixpoint/semantics/` passes through `E(N₀)`, including the vectors that catch Seam's `E_long` (`Top <- S !. ; S <- 'a' / 'ab'` on `ab` → false) and `E_troj` (a rule `Admin`, `Guard <- !'root' Name` on `root` → false);
- c) the seed witness produces the same `addr(N₀)` on the `L₀` subset both cover.

**T12. JSON through the engine (mirror-001's tests).** Two JSON events differing only in key order and whitespace → same `root`; changing one field value → different `root`; a duplicate key → `verdict partial` with a `\`. No `serde` symbol is linked into the compile path: `nm` on the release binary, filtered to the compile module, shows no `serde_json`.

**T13. One engine.** `mirror pact eval` vectors 01, 03 and 04 pass through `E`'s core, and semi-naive equals naive on every vector (math Prop. 5.4). The over-side retraction of math Prop. 5.5 appears as a deposited status splinter, not a deletion.

**T14. Rulers (after `@ruler` lands).** `settle … by:` with a bare number is `Opaque` ("whose ruler?"); `settle` with no `by:` is a compile error; an `@ruler` reference in a `focus` or `shift` position is a compile error.

**T15. One Splinter (prism side; forwarded to prism-Reed).** *[Seam's repair]* Two **consecutive** ticks whose states agree **modulo a common phase rotation** deposit one splinter twice: one address, two deposit records (math Prop. 1.7, Def. 1.8). The same splinter deposited at ticks `t` and `t + p`, `p > 1`, with different splinters in between, is reported as `cycle(p)`, never as fixed (Prop. 1.9).

**T17. One shard, its interface, its lint** *[10-29]*.
- a) `mirror compile a.mirror` in a directory where `a.mirror`'s `in @b` target does not exist prints the same record as in a directory where it does: compile reads no other shard (run under a sandbox that denies reads outside `a.mirror` and the grammar).
- b) Changing an action's **body** in `a.mirror` changes `root` and leaves `interface` unchanged; changing its signature or a `requires` changes both.
- c) A code line `x = @tool/git.commit()` with no `in @tool/git`: `verdict partial` (a Dark line), exit 1, `undeclared 1`, one `undeclared @tool/git` record, and `@tool/git` in the interface's `undeclared` part (so `interface` differs from the same shard with the `in` line added). The same reference in a comment: `verdict success`, `undeclared 0`. With `in @tool` only (prefix): still `undeclared 1` (row 21).
- d) **Corpus fixture** *[Seam 12, 13]*. The undeclared `(S, X)` **pairs** under `shards/` at `346253b`, produced by `tests/kintsugi/oracles/golden_k0.py` (independent of the compiler; comment tokens `#`, `//`, `--`, the ones the grammar must declare): **44 pairs in 33 shards** (`golden_k0.shards.out`). The earlier 35 (Mara, `#` only) and 27 (Taut) used other lexical rules. The test pins the rule and the unit; it is part of kintsugi's golden test K0 (`mirror-kintsugi.md` §9).
- e) *[Seam 13]* The same bytes **without `out`** at two paths: equal `root`, `verdict partial`, one `name?` hole. (T17a's sandbox can't catch a path dependence, because the path is not a read.)
- f) A grammar exporting rules `a` and `b`: `--start @g.a` and `--start @g.b` on the same bytes give two records with different `start`, each cached under its own key; an exported keyword is never accepted as `--start`.

**T16. Re-hash on read.** Corrupting one git object under `refs/mirror/root/<addr>` (via a replacement object) makes the load dark; it never yields a different `N`.

---

## §14 Pickup

The order for the builder: T1 red, the seed (decision 9), `E`'s core with the PEG clauses (*[10-29]* driven by §4.1's E1–E7, which need no parser), `mirror.mirror`, then T2's swap assertion green, then T1 green (T2 gates T1). T3–T7 next, because they pin meaning. T8–T13 after that. T14 waits for `@ruler`; T15 belongs to prism.

*[10-29]* T17 (one shard, interface, lint) lands with T7, since both are records compile prints. Gluing (`mirror kintsugi`, K1–K9) starts after T17 is green: kintsugi consumes interfaces. Its own order is `mirror-kintsugi.md` §9. The §5.0 bytes and §7 widths are pinned now, so T1.5's final re-hash can be written against them.

*[10-29, second pass]* v0 of kintsugi is a full run (`mirror-kintsugi.md` §0), so compile's cache is an optimisation v0 doesn't need; its key is pinned anyway (§1). T17d is re-pinned (pairs, three comment tokens), and T17 gains e (path) and f (start). Before the gate runs on the new exit table, the three docs listed under §1.1's exit codes get their edit.
