# Audit: the engine core `E`, what Rust may hold

*2026-10-29. Seam. Pre-audit, before any code. Branch `seam/audit-engine-core` from `reed/ship` @ `4e8c3d6c`.*

**What this audit is for.** The FROZEN guard (`.githooks/commit-msg`) admits a `.rs` file under `rust/*/src/` only with `[substrate-floor:@io-boundary]` plus a cited `docs/audits/YYYY-MM-DD-*.md` or a Seam sign-off. Every commit of `E` (fixpoint spec §4) will cite this file. So this file decides what is admissible, and §5 states the terms of the citation.

**Read.**
- `docs/specs/mirror-fixpoint-parser.md` §4 (the engine), §5.0 (bytes), §12 rows 7, 9 and 19.
- `docs/math/2026-10-28-mara-mirror-fixpoint-parser-math-foundation.md`: Props 1.7, 1.9, 5.1, 5.4 and 5.5, §5.4, Prop 6.4.
- My `2026-10-28` crowbar: E_long and E_troj.
- `rust/Cargo.toml` and the four path crates.
- The hook.
- `1324aae8` (`rust/src/pact.rs`, `tests/pact/*`). It is on `reed/mirror-006-pact` and `reed/mirror-fixpoint-parser`, **not on `reed/ship`**.
- Taut's `2026-10-28-taut-commit-feedback-loop-under-a-minute.md` (lever #4, workspace).

Mara's §4.1 I/O contract has not landed (`mara/t1-ambiguities` = `reed/ship` at writing). Where this audit depends on it, it says so (§3.6).

**Substrate-already-had-the-word check.** No Datalog evaluator, semi-naive loop or union-find exists in `rust/` or in `prism/` (grep `datalog|semi_naive|UnionFind|union_find|least_fixed` over both: 0 hits). `prismqueer::spectral::rotation` shares only the deposit-delta-compare shape and none of the guarantees (math §5.4). The `@pact` shards declare pacts with `\` bodies, and nothing runs them. So there is nothing to compose over.

---

## Verdicts

| # | question | verdict |
|---|---|---|
| 1 | Is `E` floor? | **ADMIT-NARROWED.** The evaluator core is floor. Everything that can be a rule program run *by* the core is not (§1) |
| 2 | Where `E` lives | **ADMIT: a new library crate `rust/engine`**, a path dependency of `mirror`, with no dependency beyond `blake3` (§2) |
| 3a | Semi-naive positive Datalog, empty-delta stop | **ADMIT** |
| 3b | Deposits, tick out of the address | **ADMIT-NARROWED.** An in-memory deposit log, with atom addresses per §5.0 only once the atom content encoding is pinned (§3.6). No git projection |
| 3c | `(e, i)` index + Ford functionality | **ADMIT-NARROWED.** The index and the assertion as a property of the model. A violation is a reported defect, never a panic. Tested on hand-written Horn clauses, not through a PEG front end |
| 3d | WF wrapper for `@pact` (two core calls per round) | **ADMIT-NARROWED.** Alternating fixpoint, safety check, retractions as status deposits. **Not**: why-not trees, canonical JSON, the `mirror pact eval` verb, or any line of `1324aae8`'s `pact.rs` |
| 3e | Union-find for kintsugi | **REFUSE** for this slice (§3.5) |
| 3f | PEG front end, any parser, serde, JSON | **REFUSE** (waits; serde never) |
| 4 | Test surface | **ADMIT-NARROWED**, as in §4 |
| 5 | Citation contract | §5 |

---

## §1 Is `E` floor?

The FROZEN rule is that capability isn't Rust unless it is the irreducible floor that runs the substrate. The hook's own history names that floor "parser, evaluator, @io boundary implementation" (`.githooks/commit-msg`, 2026-07-15 block).

**For.**
- A shard body can't run before something runs shard bodies. Today's `rust/` can't evaluate a rule or run a shard as a parser (fixpoint §12 row 9, the reason the seed is boundary Rust).
- "Compose over `@io` in a shard" has no meaning for the thing that executes the composition.
- The spec names `E` the trust root (math Prop 6.4: the fixed point certifies the seed against `E`, and nothing about `E`).
- An evaluator that runs the substrate is the textbook case of the floor.

**Against, and the narrowing.** Prop 6.4's own consequence is "DDC applies to `E`'s source: build `E` from its shard with two independent floors". So the endgame is `E` **as a shard**, with the Rust `E` as one of the floors that runs it. That fixes the boundary:

- **Floor (admissible):** the smallest evaluator for positive Datalog over a finite domain, plus the one non-monotone loop the spec needs on top of it (the well-founded alternation). It is irreducible because without it no rule runs. It must stay small, because a DDC needs a second independent implementation, and every line here is a line that second implementation must match.
- **Not floor:** anything expressible as a rule program run by the core. That covers:
  - PEG → Horn clause compilation (math §5.1 is a syntax-directed translation; once a reader exists it is a rule set);
  - why-not / provenance trees (a query over the model);
  - the `undeclared` / `gap` lints (spec: "its rules");
  - connected components (reachability is positive Datalog);
  - the `undefined`-in-the-cone count (a query over the PEG model).

  Some of these may *later* need Rust for speed. Each such case needs its own audit with a measurement, not this one.

**On the marker's name.** `[substrate-floor:@io-boundary]` was coined for `@io` primitives. `E` is not `@io`: it reads no files, opens no sockets and calls no tool. It is the *evaluator* floor. The marker is the only gate the hook offers, so `E` commits use it, and the commit body must say "floor: evaluator (not `@io`), per <this audit> §1". Whether the hook should grow a second marker (`[substrate-floor:evaluator]`) is Alex's call. I lean yes: a marker that says the true thing is cheaper than a paragraph explaining a mismatched one.

**Purity is part of the admission.** `E` does no I/O, no threads, no `unsafe`, no clock, no environment and no randomness. Output order is deterministic (`BTreeMap`/`BTreeSet`, or sorted vectors; never a `HashMap` iteration order). A commit that breaks any of these is outside this audit.

---

## §2 Where `E` lives

Options considered:

| option | testability | coupling | verdict |
|---|---|---|---|
| inside the `mirror` bin (`rust/src/`) | integration tests in `rust/tests/` can't reach a bin's internals. Only `#[cfg(test)]` unit tests can, and those sit in `src/` and so are FROZEN (the hook exempts only `tests/`). TDD's red-first test would itself need the floor marker | inherits `prismqueer` + LAPACK + `serde_json` (root `Cargo.toml` depends on `serde_json = "1.0"` despite its own "NO serde" comment) | refuse |
| inside `rust/spectral` | lib, so testable | pulls `prismqueer` with `lapack, bundle`. A Fortran/LAPACK build sits under the trust root and DDC must reproduce it. Spectral is the liquid/spectral reading, not an evaluator | refuse |
| inside `rust/fractal` | lib, so testable | `prismqueer` + `blake3`. Fractal is identity, subject and signing. Putting the evaluator next to signing keys widens the trust root for no gain | refuse |
| binary-only (tests through a CLI verb) | needs an input wire format before any reader exists, which drives toward JSON input, i.e. serde, the thing §9 forbids. Circular | — | refuse |
| **new lib crate `rust/engine`** | integration tests in `rust/engine/tests/` call the public API. That path is **hook-exempt** (`^rust/([^/]+/)?tests/`), so red tests need no marker, and only `src/` needs this audit | depends on nothing but `blake3` (already in `Cargo.lock` via `fractal`, and in the root's dev-dependencies). `serde` absent from its `Cargo.toml` is a checkable "no serde" | **admit** |

**Why `rust/engine` fits the layout.**
- **Five-crate layout.** It is a sixth path crate, with its own `Cargo.toml`, a member of nothing, and a path dependency of `mirror`. That is the same shape as `fractal`, `matrix`, `roomba` and `spectral`. The root comment's "NOT a workspace member, until post-dock workspace lift MAY land" still holds.
- **Taut's lever #4 (workspace).** It is orthogonal. A dependency-free path crate lifts into a workspace unchanged. With no `prismqueer` it adds one small lock graph and a fast test binary to the interim five-`cargo test` gate. Taut's numbers make the cold `prismqueer`+Fortran builds the cost, and `rust/engine` adds none.
- **DDC.** A crate with one dependency is the smallest artefact to double-compile and to re-implement independently.
- **The name.** `engine` is a placeholder. The spec says "the engine `E`" and "one rotation core". Alex names primitives. A rename before the first `src/` commit costs nothing; after, it is a mechanical move.

**The crate's API is the contract surface.** The public API (rules, atoms, evaluate, deposits, the WF wrapper) is exactly what Mara's §4.1 pins. Nothing else is `pub`. `mirror` calls it; no verb is added in this slice.

---

## §3 The first slice

### §3.1 ADMIT: the semi-naive core

- Positive Datalog, function-free, over a finite domain of constants (math §5.2). A tick is a round, and only the delta joins (each rule instance must use at least one delta atom). It stops at the first empty delta (Prop 1.9: the cumulative test, right for monotone steps).
- Termination bound: ≤ |B| ticks (Prop 5.4). The core asserts it, and a run that exceeds it is a defect report, not a loop.
- Rule and fact order never change the model (T-permute, §4).

### §3.2 ADMIT-NARROWED: deposits, tick out of the address

- Each newly derived atom is deposited as `(addr(atom), tick)`. The tick lives in the deposit record and **never** in `addr` (Prop 1.7).
- The store is **in-memory and append-only**. Git projection (§7), refs and on-disk storage are not this slice. They are `@io`, and they get their own audit.
- **`addr` is §5.0's splinter rule**, `H("splinter\0" ‖ τ ‖ enc(content))`, with τ for `@parse/item` and `@pact/fact`. §5.0 pins `τ`, `enc` and `H`. It does **not** pin `content` for a Datalog atom (how `m(e, i, j)` or `send(a1)` becomes bytes). Until Mara's §4.1 (or a §5.0 row) pins it, `E` may compute addresses, but **no test may pin the bytes of a derived atom's address**. Tests assert the properties (tick-independence, injectivity on distinct atoms, equality under re-derivation), never hex.
- Fragment and lens addressing are not `E`. They belong to normal-form addressing (T1 already re-hashes in its own test).

### §3.3 ADMIT-NARROWED: the `(e, i)` index and Ford functionality

- An index on `m` by `(e, i)`, and the assertion "at most one `j`" (Ford functionality) checked on the model. A violation is a **reported defect** (a status deposit naming `(e, i)` and both `j`s). It is never a panic, and never silently first-wins.
- Tested on **hand-written Horn clauses** of math §5.1 for tiny grammars, as test data. Example: `S <- 'a' / L ; L <- L 'b'` on `a` gives `success` and `wf false`; the clauses are transcribed, not compiled. The PEG → clauses translation waits (§3.5).
- The linear-time claim stays a CONJECTURE (spec §4, T6b). No benchmark gate in this slice.

### §3.4 ADMIT-NARROWED: the well-founded wrapper for `@pact`

**Admitted:**
- `gamma(J)` = the least model of the program with negative literals evaluated against the fixed context `J`. That is one core call, positive by construction.
- The alternation: `under` ascends and `over` descends, two core calls per round (math §5.3, Prop 5.5). It stops when `under` repeats consecutively. The outer loop is non-monotone in the pair, so it uses the consecutive-equality test of Prop 1.9, never the cumulative one.
- Statuses: true / false / undefined, with "closed world for facts only" (`tests/pact/oracles.md`).
- Safety check (range restriction): an unsafe rule is a `dark` verdict naming the variable, never a panic.
- Retractions on the `over` side are deposited as status splinters.

**Not admitted in this slice:**
- Why-not trees (vector 03's `because`): a query over the model, so a rule program, so not floor.
- Canonical JSON output, and the `mirror pact eval` verb.
- **Any line of `1324aae8`'s `rust/src/pact.rs`.** It is built on `serde_json::Value` (`use serde_json::{json, Value}`, line 31) and on naive iteration. The WF semantics are re-implemented against `E`'s core. That commit never reached `reed/ship`, and nothing here admits merging it.

### §3.5 REFUSE (this slice)

- **Union-find for kintsugi.**
  - Components are reachability, which is positive Datalog over the core.
  - The only non-Datalog part is a count (b₀, b₁ = |E| − |V| + b₀), which is a length of a set of representatives.
  - Kintsugi's spec is Red, gated behind compile T17 (kintsugi §9 K1).
  - My compile/kintsugi crowbar (`2026-10-29-seam-crowbar-compile-kintsugi.md` §3) shows R7 needs Σ's global component map, and deletions need dynamic connectivity.

  That design is unsettled. Union-find enters Rust only with its own audit, a measured need, and a settled kintsugi locality story.
- **The PEG front end, the seed `P₀`, any parser or reader.** The seed is boundary Rust under its own audit (row 9: "Seam's audit file landing before the first green commit"). That audit is not this one. The PEG → Horn translation and the `undefined`-in-the-cone count wait for the front end.
- **serde, ever.** `rust/engine/Cargo.toml` carries no `serde`, `serde_json` or `serde_derive`, directly or as a feature.
- **JSON.** T12 goes through the engine later (`E(G_json)`), never through a JSON library.
- **Kuramoto, coupling, Fiedler** (spec §4 "Not in the core").

### §3.6 Dependency on Mara's §4.1

The first `src/` commit may land before §4.1, under the narrowings above. Once §4.1 lands:
- if it pins atom content bytes, the address tests may pin hex;
- if it changes the API shape (e.g. how negative context enters `gamma`), the change is inside this audit **provided** it stays within §3.1–§3.4;
- if §4.1 adds a capability not listed in §3.1–§3.4, that capability needs a new audit.

---

## §4 The admissible test surface

All tests live in `rust/engine/tests/` (hook-exempt) and use only `E`'s public API. They are red first.

**Fixtures are data.** They are Rust literals in the test file, or mirror-text read with a hand-written line splitter in the test. Never JSON through serde. Never `include_str!` of a `.json` file parsed by anything.

**ADMIT:**
1. **The pact vectors.** 01 (transitive closure with a cycle; the true set; permutation invariance), 04 (negation cycle: `p`, `q`, `t` undefined, `r` true, outcome opaque), and **03 with statuses only** (`send(a1)` false, `ready(a1)` false, `answered(…)` absent; the `because` tree is not asserted).
   - They are transcribed as Rust literals, citing `tests/pact/oracles.md` and the vector names, plus the blob ids from `1324aae8` (`git ls-tree 1324aae8 tests/pact/`) so the transcription is checkable.
   - The oracle stays `oracles.md`. If transcription and oracle disagree, the oracle is right.
   - `oracles.md` and the three `.json` files are not on `reed/ship`. Bringing `oracles.md` over (markdown, 📝) is fine; the `.json` files may come as reference data, read by no test.
2. **Semi-naive = naive (Prop 5.4) as a property test** against an **independent naive evaluator written in the test file**.
   - "Independent" means written from math §5.2, not adapted from `E`'s source. The commit says so.
   - This evaluator is the first diverse implementation for Prop 6.4's DDC of the core semantics, so its independence is load-bearing.
   - Random programs come from a fixed-seed generator in the test (an inline LCG; no `rand` dependency). At least 1000 programs, and they include recursion, mutual recursion and rules with repeated variables.
3. **T-permute.** Every permutation (or a fixed stated sample) of rules and facts gives an identical model and identical deposit **sets**. Tick values may differ only if the spec says so; today they may not, because semi-naive rounds are order-independent.
4. **T-rename (the E_troj lesson at core altitude).** A consistent bijective renaming of predicate and constant symbols commutes with evaluation. A core whose behaviour depends on a symbol's *name* (E_troj fires on a rule named `Admin`) fails it. Run it with a renaming that maps some symbol *to* `Admin`, and one that maps `Admin` away.
5. **Prop 1.7.** Re-deriving an atom at a later tick deposits nothing new, the run stops, and `addr` of an atom is equal across two runs where it falls at different ticks.
6. **Prop 1.9.** The WF outer loop on a program whose naive `over`/`under` would oscillate if mis-stopped stops correctly. Prop 5.5's example (`b :- not a. c :- not b. d :- not c.` with fact `a`) retracts `d` and deposits the retraction.
7. **Ford functionality** on hand-written clauses (§3.3). Include one deliberately non-functional clause set, and assert the defect report rather than a panic.
8. **Safety.** An unsafe rule gives `dark` naming the variable, with no panic.

**E_long and E_troj.** E_long (ordered choice read as longest match) is a PEG-front-end misreading and needs the front end and the semantics oracle, so it is **not** in this slice. E_troj's *grammar* vector also needs the front end. Its **lesson** is in this slice as T-rename (item 4), because a name-conditioned branch can live in the core. Both full vectors go to the semantics oracle with the PEG slice, and that slice's audit must list them.

**NOT admissible:** a test that pins derived-atom address hex before §4.1/§5.0 pins content (§3.2); a test that reads JSON; a test that exercises a CLI verb.

---

## §5 The citation contract

The hook only greps for *some* `docs/audits/YYYY-MM-DD-*.md`. It does not check which audit, or that the commit is inside it. So the contract is enforced by review, and by Seam reading `git log --grep engine-core`. An `E` commit that touches `rust/engine/src/**` cites honestly **iff its message contains all of**:

1. `[substrate-floor:@io-boundary]` and the line `Floor: evaluator (not @io), within docs/audits/2026-10-29-seam-engine-core-scope.md §<n>`, where §<n> ∈ {3.1, 3.2, 3.3, 3.4} names the item(s) the diff implements;
2. the red test(s) it turns green, by file and test name, under `rust/engine/tests/`. The red commit precedes it (TDD; sequential commits);
3. `No serde. No I/O. No new dependency.` The commit may add only `blake3`, and only once;
4. for the commit that adds the naive oracle (§4 item 2): `Naive evaluator written from math §5.2, not from E's source.`

Outside the crate, the only admitted non-test edits are `rust/engine/Cargo.toml` (new), a `[dependencies] engine = { path = "engine" }` line in `rust/Cargo.toml`, the lockfile, and the gate/Justfile line that runs `rust/engine`'s tests.

**What puts a commit outside this audit (it needs a new one):**
- any `.rs` under `rust/engine/src/` that implements something not in §3.1–§3.4: union-find or any graph algorithm; a PEG front end, reader, lexer or seed; why-not/provenance trees; JSON or mirror-text printing; git projection or any file, socket or process I/O; Kuramoto or spectral anything;
- a new dependency (anything beyond `blake3`), or any `serde*`, `rand`, `regex` or `unsafe`;
- `pub` API beyond the §4.1 contract;
- any edit to `rust/src/**` (the `mirror` bin), including wiring a verb to `E`. Wiring `mirror compile` or `mirror pact eval` to `E` is a separate floor question (it is CLI, i.e. `@io`) and needs its own audit or a Seam sign-off trailer;
- porting code from `1324aae8`'s `pact.rs`;
- changing a test fixture's expected values to match `E` (only the oracle may correct a fixture, with a dated note in `oracles.md`).

**Expiry.** This audit covers `E`'s first slice only. When the PEG front end, the seed or kintsugi's union-find arrive, they cite their own audits. The "Floor:" line in a commit is a claim I will check against the diff. A wrong claim is a finding, not a technicality.

---

## §6 For Alex

1. The crate's name (`engine` is a placeholder).
2. A second hook marker `[substrate-floor:evaluator]`, so `E` commits stop wearing `@io` (lean: yes).
3. Hook tightening: require the citation to name an audit **and a section**, e.g. `docs/audits/…\.md §[0-9]`. The hook can't check scope, but it can stop a bare citation (lean: yes).

*— Seam*
