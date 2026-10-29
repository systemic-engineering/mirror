# Audit: `mirror pact eval` and the ruler envelope

*2026-10-29. Seam. Pre-audit, before any code. Branch `seam/audit-pact-eval` from `reed/ship` @ `1ae650de`.*

**Scope.** This audit covers two slices:
- the first end-to-end verb, `mirror pact eval`, which reads bytes, runs the engine's well-founded wrapper `W`, and prints a record;
- the ruler envelope in Rust (`docs/specs/mirror-ruler.md` §1, §5, §6; math §8–§11).

Commits in either slice cite this file. §3 says how. My engine-core audit (`2026-10-29-seam-engine-core-scope.md`) still governs `rust/engine/src`. Nothing here widens it, except the one engine-adjacent rule in §2.3.

**Read.**
- `rust/engine/src/lib.rs` (1173 lines). The public API: `Fact`, `NormalRule`, `Literal`, `NormalProgram::new` (arity, safety and declaration checks giving `Dark`), `Instance::new`, `well_founded → WellFounded { true_atoms, undefined, retractions, outcome, findings }`, `WellFounded::status`, `Budget(NonZeroU64)`, `run_bounded`.
- `rust/engine/tests/e7_no_serde.rs`.
- `rust/Cargo.toml`. The bin depends on `serde_json` (used at `rust/src/main.rs:469-551`, the MCP serve path) and does **not** yet depend on `engine`.
- The pact vectors and `oracles.md` at `5babfbad` (`reed/mirror-006-pact`).
- `docs/specs/mirror-ruler.md` §1 (envelope + family table), §5 (D1–D7: D2 and D6 decided, D1, D3, D4, D5 and D7 open).
- `fractal`'s signing surface (`subject.rs:33`: signing discharges through `@tool(ssh-keygen)`).

---

## Verdicts

| # | item | verdict |
|---|---|---|
| 1.1 | `mirror pact eval` as a bin verb | **ADMIT-NARROWED**: one file in, one record out, no other I/O (§1.1) |
| 1.2 | A hand-written reader for a pact-text subset, before T1 | **ADMIT-NARROWED**: the exact grammar in §1.2, frozen. It retires when `E(G_pact)` reads the same fixtures. It is never extended in place |
| 1.3 | Waiting for T1 instead | not required. The reader is small and bounded, and it is replaced, not grown |
| 1.4 | Output record and exit codes 0/1/255/2 | **ADMIT** as §1.4 |
| 1.5 | Vectors 01/03/04 through the binary | **ADMIT-NARROWED**: 03 asserts statuses only, never `because` (§1.5) |
| 1.6 | `serde_json` in the slice | **REFUSE**: no new `serde` use anywhere in this slice. Removing the dependency is a later end-to-end |
| 2.1 | Ruler envelope home | **ADMIT a new lib crate `rust/ruler`**, depending on `engine` (for `Addr`, `Tick`, `Budget`) and `blake3` only (§2.1) |
| 2.2 | Envelope contents in this slice | **ADMIT-NARROWED**: `Ruler<B>`, sealed `Body`, the `Count` body only, `Shaped<T, E>`, `Approval::Unsigned` recorded. **No `Approved` constructor and no `verify`** (§2.2) |
| 2.3 | `Budget` as the first ruler-typed parameter | **ADMIT-NARROWED**: by wrapping, not by changing the engine. `ruler::run_bounded(&Instance, &Ruler<Count>) → Shaped<Run<Bounded>, Truncated>`. The bin may not call `engine::run_bounded` directly (§2.3) |
| 2.4 | Signing, verification, keys | **REFUSE** in this slice, pending D1 and D7 and a key audit (§2.4) |
| 2.5 | `Observe` / `Restrict`, the restricted run | **REFUSE** (D3 open; outside both audits) |
| 3 | Citation contract | §3 |

---

## §1 `mirror pact eval`

### §1.1 What it is, and why it may be boundary Rust

The verb reads **one file**, given as a path argument, into a `NormalProgram`, `Facts` and a list of queries. It runs `engine::well_founded` once and prints one mirror-text record to stdout.

Reading bytes into facts and printing a record are the `@io` boundary in the marker's original sense: unlike my engine-core audit §1, the marker fits here without a footnote. Everything between reading and printing is the engine's.

What the verb **does not do**:
- read any file but its argument (no includes, no `in @X`, no config, no env, no cwd scan);
- write any file, deposit to a store, or touch git;
- open a socket or spawn a process;
- compute why-not trees (vector 03's `because`; a query over the model, so clause-side, as in the engine audit §3.4);
- emit JSON;
- call anything in `serde_json`.

Code location: one new module in the bin, `rust/src/pact_text.rs` (reader + printer), plus a dispatch arm and a `VERBS` row in `main.rs`. The name avoids `pact.rs`, which is the unmerged `1324aae8` file on `reed/mirror-006-pact`. **No line of `1324aae8` is ported.**

### §1.2 The reader's grammar (bounded, frozen)

This is the whole language. Anything else is `dark` at its line and column, exit 2.

It is chosen so that it is a **subset of the surface the vectors already show** (their `text` fields: `h requires b₁, b₂, not x`), so that the same fixtures stay valid when `E(G_pact)` reads them. Mara must confirm that the subset is a sublanguage of mirror's pact surface before the first reader commit. Otherwise the fixtures would pin a dialect.

```ebnf
file      = { line } ;                         (* bytes: ASCII 0x20-0x7E and LF only; CR, TAB, NUL, non-ASCII: dark *)
line      = ( blank | comment | record ) , LF ;  (* the last LF may be missing *)
blank     = { " " } ;
comment   = "#" , { any-printable } ;          (* only at column 1; no trailing comments *)
record    = key , sp , body , { " " } ;
sp        = " " , { " " } ;

key  "rule"       body = rule-id , sp , atom , sp , "requires" , sp , literal , { "," , sp0 , literal } ;
key  "fact"       body = ground-atom ;
key  "query"      body = ground-atom ;
key  "functional" body = pred , sp , position , { "," , sp0 , position } , sp , "->" , sp , position ;

literal     = atom | "not" , sp , atom ;
atom        = pred | pred , "(" , term , { "," , sp0 , term } , ")" ;   (* zero-ary: no parentheses; "p()" is dark *)
ground-atom = pred | pred , "(" , sym , { "," , sp0 , sym } , ")" ;
term        = var | sym ;
var         = "?" , name ;
pred        = name ;                           (* not "not", not "requires" *)
sym         = name ;                           (* not "not", not "requires" *)
rule-id     = lower , { lower | digit | "_" | "-" } ;
name        = lower , { lower | digit | "_" } ;
position    = nonzero-digit , { digit } ;      (* a Position (1-based argument index), never a value *)
sp0         = { " " } ;
lower = "a".."z" ;  digit = "0".."9" ;  nonzero-digit = "1".."9" ;
```

Rules the grammar can't say, checked by the reader (each failure is `dark`, naming the line):
- **Rule ids are unique.** Provenance names rules by id.
- **Queries are ground, and their arity agrees with the program's signature.** `Instance::new` checks facts but not queries, so the reader does it.
- **No numeric constants.** Every constant is a `sym`. `Const::Nat` and `Const::Addr` are unreachable from this reader, so the number-type question (compile §12 row 18) doesn't enter. `position` is an argument index, not a datum.
- **Duplicate `fact` lines are allowed** (a set). Duplicate `query` lines print once.
- **Line order is not meaning.** The output is byte-identical under any permutation of record lines (§1.5).
- **Engine `Dark`** (`Unsafe`, `Arity`, `Functional`) is printed as a `dark` record, exit 2, never a panic.
- **No size limit** in the reader. A limit is a narrowing parameter (R1); if one is needed later, it is a `Count` ruler.

The reader is linear, single-pass and allocation-bounded by the input. It has no recursion, no regex dependency, and no lookahead beyond one line.

**Retirement.** When T1 is green and `E(G_pact)` exists, the verb reads through it, and the same fixtures must produce the same `Instance` through both readers before the hand reader is deleted. That is a diverse double read, in the spirit of Prop 6.4. **Extending this grammar (new keys, numbers, trailing comments, includes) is outside this audit.**

### §1.3 Why not wait for T1

Waiting costs the first end-to-end and buys nothing the retirement rule doesn't. The risk of a hand reader is that it grows into a second parser. The frozen grammar plus the retirement condition is the guard, and the citation contract makes growth a new-audit event.

### §1.4 Output and exit codes

Mirror-text, `<key> <value>` lines (kintsugi-ci §1.4). The summary comes first, then one line per atom, then findings. Atoms are printed in the oracle's surface form (`path(a, b)`) and sorted by that text, the oracle's "lexicographic, element by element" order, never by address.

```text
verdict     clear | opaque | refused | dark
file        "<path as given>"
program     <addr: H over the sorted rule records, 64 hex>   (see note)
facts       <addr of the fact set, 64 hex>
rounds      <W's outer rounds to consecutive equality>
shaped_by   none
true        <n>
undefined   <n>
findings    <n>

true        path(a, b)
...
undefined   p
false       send(a1)             # queried atoms only
finding     functional <pred> key(<syms>) <least> <other>
```

- **`true` and `undefined`** list every atom whose predicate is **intensional** (a rule head), per `oracles.md` ("`true` and `undefined` always list every derived (intensional) atom"). Facts of extensional predicates are not listed. `false` lists only queried atoms.
- **`shaped_by none`.** `W` runs to its fixed point (total, no budget), so no ruler shaped this result. The field is printed so its absence is a statement. It becomes non-empty only when a ruler parameter exists (§2).
- **`program` / `facts` addresses.** The engine's `addr` is per fact. A program address needs a pinned encoding of rules (Mara's §4.1 territory). Until it is pinned, these two lines are **omitted**, not invented. A test may not pin them.

| verdict | when | exit |
|---|---|---|
| `clear` | `Outcome::Clear` and no findings | 0 |
| `opaque` | `Outcome::Opaque` (something undefined), no findings | 1 |
| `refused` | any `FunctionalViolation` on `true` (a definite violation of the program's own declaration) | 255 |
| `dark` | the bytes don't read (§1.2), or the engine returns `Dark` | 2 |

This follows Alex's oriented ternary as `mirror love` uses it (0 / 1 / 255), plus 2 for dark, as `strict-and-total` specifies. A queried atom that is **false** is a clear answer (`oracles.md`, vector 03: "a definite no with its reason"), not a refusal. Usage errors (no path, unknown flag) print to stderr with no record and exit 2. They are distinguishable from dark by the absent record.

### §1.5 Tests

They live in `rust/tests/pact_eval.rs` (hook-exempt) and drive `env!("CARGO_BIN_EXE_mirror")`. They are red first.

**Fixtures.** `tests/pact/0{1,3,4}-*.pact-text` are transcribed by hand from the `.json` vectors at `5babfbad`. Each fixture's header comment cites the source blob id (`git ls-tree 5babfbad tests/pact/`). Expected outputs are `tests/pact/0{1,3,4}-*.expected` in the §1.4 format. `tests/pact/oracles.md` comes over to `reed/ship` (📝).

**The `.json` files do not come over**, so no test can be tempted to read them. If transcription and oracle disagree, the oracle is right, and corrections are dated in `oracles.md`.

**Cases:**
1. **Vector 01:** exit 0, `clear`, the nine `path` atoms. Then all permutations of its record lines (2 rules × 4 facts, 48 orders) give byte-identical stdout.
2. **Vector 03:** exit 0, `clear`, `false send(a1)`, `true 0`. **No `because` assertion.** `answered(a1, alex, yes)` is "absent" in the oracle; this slice prints only queried atoms, so absent vs false doesn't arise.
3. **Vector 04:** exit 1, `opaque`, `true r`, `undefined p`, `undefined q`, `undefined t`, plus permutation invariance.
4. **`refused`:** one new vector, `05-functional-violation`, is written into `oracles.md` by Mara **before** its test. `oracles.md` reserves 05 for dark, so Mara picks the number. Expected exit 255.
5. **Dark, reader:** one case per grammar rule broken. Each prints a `dark` record with line:col and exits 2, with no panic: CR, TAB, non-ASCII, trailing comment, `p()`, a numeric constant, `not` as a predicate, a duplicate rule id, a non-ground query, query arity mismatch, an empty `requires`.
6. **Dark, engine:** an unsafe rule (`p(?x) requires not q(?x)`) prints `dark` with the rule id and the variable, exit 2.
7. **No serde in the slice:** the test reads `rust/src/pact_text.rs` as text and asserts no `serde`, the same shape as `e7_no_serde.rs`.

---

## §2 The ruler envelope

### §2.1 Where it lives: a new crate `rust/ruler`

| option | verdict | reason |
|---|---|---|
| inside `engine` | refuse | the engine is the trust root and must stay minimal for DDC (engine audit §1). The envelope will carry approval, and approval will carry crypto. Neither belongs under the trust root. My engine audit's §5 also forbids `pub` API beyond §4.1 |
| `fractal`, next to signing | refuse **for now** | fractal pulls `prismqueer` + `blake3`. Its signing discharges through `ssh-keygen` (a process, i.e. `@io`). Putting the envelope there couples every ruler-typed call to prismqueer's build and to the signing path before D1/D7 decide what approval is. Verification may later *call into* a signing crate, under its own audit |
| the bin | refuse | untestable internals; the envelope is shared by the bin and future library users |
| **new `rust/ruler`** | **admit** | depends on `engine` (`Addr`, `Tick`, `Budget`, `Run`) and `blake3`, nothing else. Its tests sit in `rust/ruler/tests/` (hook-exempt). It is one more small path crate in the non-workspace layout, cheap in the gate, and lifts unchanged into a workspace. The name is a placeholder; Alex names primitives |

The dependency direction is fixed: **`ruler → engine`, never the reverse.** The engine never learns what a ruler is.

### §2.2 What the envelope may contain in this slice

**Admitted:**
- **`Ruler<B: Body>`** with private fields `{ addr, reader, body }`.
  - The only constructor takes a **parsed** body value and computes `addr = H("ruler\0" ‖ τ ‖ enc(canonical body bytes))`, using the §5.0 primitives (`H` = BLAKE3, `enc` = u64-BE length prefix).
  - The canonical bytes of `Count` are trivial: `u64 BE n ‖ unit tag`. Mara pins them in the spec before the commit; no test pins hex until then.
  - **No constructor takes an `Addr`.**
- **`Body`, sealed** (a private supertrait), with exactly one implementation in this slice: **`Count { n: NonZeroU64, unit: Unit }`** with `Unit = Ticks | Rounds` (the engine's `Tick` and `Round` are distinct types already).
  - Sealing the *body set* is independent of D3, which is about `Observe`/`Restrict`. Unsealing later is non-breaking, and sealing later would be breaking, so seal now.
- **`ReaderDependence`**: `Count` admits only `Independent` (spec §1 table). The constructor refuses `Dependent` for `Count`, as a `Result`, not a panic.
- **`Shaped<T, E>`** with `#[must_use]`, private fields, and getters:
  - `shaped_by: NonEmpty<Addr>`;
  - `observed_by: Vec<Addr>` (always empty in this slice);
  - `erasure: E`;
  - **`approval: Approval`**, where `Approval` has one variant in this slice: `Unsigned`.

  This is the same honesty as `active unsigned` in the compile record (fixpoint §8): the ruler is real and addressed, and nobody has signed it yet, so the result says so.
- **`Truncated { at: Tick }`** as `Count`'s erasure. It is `None`-like when the run reached its fixed point within the budget. The engine's `Bounded` tells which happened.

**Not admitted in this slice:**
- **`Approved`**: no constructor at all. The type may be declared for documentation with no way to build it. Nothing in this slice consumes it.
- **`verify`**, **`Keyring`**, **`ApprovalChain`**, **`Revocation`**.
- **Other bodies**: Choice (clause-side by D2/D6, so not a Rust body at all), Resolution (`Band` + `ScaleLaw`: acts on prism readings, and its scale laws need Mara's pinning), Equivalence, Authority (needs a signer, so keys), Identity root.
- **`Pact`/`RulerFractal`**, checks, shape (D5 open; nothing consumes them).
- **`Observe`, `Restrict`, `run_restricted`, `well_founded_restricted`** (D3 open).

### §2.3 `Budget` as the first ruler-typed parameter: wrap, don't rewire

The engine keeps `run_bounded(&Instance, Budget)`. It is the unshaped core, admitted by the engine audit, and `Budget(NonZeroU64)` there is a *bound*, not a policy record.

The ruler crate adds:

```rust
pub fn run_bounded(inst: &Instance<'_, PositiveProgram>, budget: &Ruler<Count>)
    -> Result<Shaped<Run<Bounded>, Truncated>, UnitMismatch>;   // Count must be in Ticks
```

It converts `Count` to `engine::Budget` and calls the core. The result's `shaped_by = [budget.addr]` and its erasure is `Truncated { at }` iff `Bounded::Budget`.

**R1 holds at the API the bin uses, not inside the engine:**
- **The bin may not call `engine::run_bounded` or `engine::Budget::new`.** A test in `rust/tests/` reads `rust/src/**/*.rs` as text and asserts neither string appears. This is the checkable form of R1 for this slice.
- `pact eval` uses `W`, which takes no budget, so the slice's verb doesn't use the ruler crate. The two items are independent and may land in either order.
- `Instance::sufficient_budget` is a theorem-derived bound (§4.1 A3, PROVED), not a ruler. It needs no envelope, and the record doesn't list it in `shaped_by`.

### §2.4 Keys: what may and may not be in this slice

This slice **may not** contain:
- any private key, seed or key file under `rust/`, `tests/` or fixtures;
- key generation;
- any `ssh-keygen` (or other) process invocation;
- any crypto dependency (`ed25519-*`, `ssh-key`, `ring`, …);
- any `verify`;
- any `Signed-off-by`-style approval inferred from text.

This slice **may** contain:
- `Approval::Unsigned`, recorded in every `Shaped`;
- documentation of the planned `Approved`/`verify` shape, as doc comments and not as callable items.

Signing arrives under its own audit once **D1** (approval across versions) and **D7** (the gate's checks, including "a Seam trailer requires Seam's own signature") are decided. That audit decides where keys live; my lean is `fractal`, which already owns identity. Until then nothing in Rust can approve anything, which is the honest state: today's only approval route is the commit gate, and it checks strings (D7).

---

## §3 The citation contract

It has the same shape as the engine audit §5. The hook only checks that *some* audit path appears in the message (D7 open), so the terms below are enforced by review. Seam checks `git log --grep pact-eval --grep ruler-envelope` against them.

**A commit that touches `rust/src/**` or `rust/ruler/src/**` cites honestly iff its message contains:**
1. **The marker and floor line.**
   - For `rust/src/**`: `[substrate-floor:@io-boundary]` and `Floor: @io (read one file, print one record), within docs/audits/2026-10-29-seam-pact-eval-and-ruler-envelope.md §1.<n>`.
   - For `rust/ruler/src/**`: the marker and `Floor: envelope record (no keys), within … §2.<n>`.
2. **The red test it turns green**, by file and test name. The red commit comes first.
3. **`No serde. No keys. No new dependency.`** The only admitted dependency edges are:
   - `engine` in `rust/Cargo.toml`;
   - `engine` and `blake3` in `rust/ruler/Cargo.toml`;
   - `ruler` in `rust/Cargo.toml` (only if the bin uses it).
4. **For the reader commit:** `Grammar exactly §1.2; Mara confirmed sublanguage of the pact surface: <commit>`.

**Outside this audit (needs a new one):**
- any change to the §1.2 grammar: new keys, numeric constants, trailing comments, includes, CR/TAB tolerance, a size limit as a bare number;
- reading more than the one argument file, writing any file, a store deposit, git, a socket or a process;
- why-not trees, `absent` provenance, JSON output, or any `serde*` use in new or touched code;
- porting from `1324aae8`;
- in `rust/ruler`:
  - any body but `Count`;
  - any constructor of `Approved`, any `verify`/`Keyring`/`ApprovalChain`/`Revocation`;
  - any crypto dependency or key material;
  - `Observe`/`Restrict`/restricted runs;
  - `Pact`/fractal/shape;
- any edit to `rust/engine/src/**` (governed by the engine audit, and nothing here widens it);
- the bin calling `engine::run_bounded` or `engine::Budget::new` directly;
- changing an expected output to match the binary. Only `oracles.md` corrects a fixture, with a dated note.

**Expiry.** This audit covers the first pact-eval slice and the `Count` envelope. Removing `serde_json` from the bin, signing/verification, further bodies and the restricted run each cite their own audits.

---

## §4 For Mara and Alex

**Mara** (before the first `src/` commit of each slice):
- confirm §1.2 is a sublanguage of mirror's pact surface (`requires`, `not`, `?x`, `functional … -> …`), or amend the grammar here by a 📝 to this audit;
- pin the canonical bytes of `Count` (and the `ruler\0` kind prefix) in `mirror-ruler.md`;
- write vector `05-functional-violation` (number of her choosing) into `oracles.md`.

**Alex:** nothing new blocks this slice. D1 and D7 block signing (§2.4). D3 blocks restricted runs. D5 (the word "pact") will rename the verb if the clause shards don't keep the word. The verb is `mirror pact eval` until then.

*— Seam*
