# Seam crowbar: mirror's fixpoint parser (Mara, 2026-10-28)

**Auditor:** Seam `<seam@systemic.engineer>`. 2026-10-28.
**Targets:** `docs/math/2026-10-28-mara-mirror-fixpoint-parser-math-foundation.md` ("math"), `docs/specs/mirror-fixpoint-parser.md` ("spec"), with `docs/scouts/2026-10-28-taut-graph-magic-dag-to-laplacian.md` as context.
**Code read:** prism `7e6b00f` (`~/.cache/mirror/prism-7e6b00f/prismqueer/src`: `oid.rs`, `coincidence.rs`, `spectral_uuid.rs`, `spectral/rotation.rs`, `beam.rs`, `lib.rs`; `native/spectral.f90`); prism `docs/loop/CURRENT.md` at `43f47ba`.
**Method:** I did not run or read Mara's scripts. Every COMPUTED line below comes from my own code in `~/dev/ai/seam/scratch/fixpoint-parser/` (not committed). Each finding has reproduction steps.

Verdicts: **HOLDS** · **HOLDS-WITH-REPAIR** (repair given) · **FAILS** (counterexample given) · **UNTESTED**.

| # | target | verdict |
|---|---|---|
| 1 | Prop. 7.2, `Oid::hash` | HOLDS-WITH-REPAIR: the conclusion holds; the proof's model ("a function of three f64 scalars", "at most 192 bits") is wrong; the CONJECTURE of cheap collisions is UNTESTED and my estimate is against it |
| 2 | Prop. 1.7, tick out of the address | HOLDS as stated; the step from it to T15 FAILS for `rotation.rs` |
| 3 | Prop. 4.1, choice needs `split` | HOLDS-WITH-REPAIR: the hypothesis isn't the Beam contract, and what is needed is a case split, not a fork. The `project` correction HOLDS |
| 4 | Prop. 5.1, PEG = positive Datalog | HOLDS (proof given; exhaustive check), with one spec repair on `undefined` |
| 5 | Prop. 6.2, Thompson | HOLDS-WITH-REPAIR: the trojan lives in the engine, not the seed. Spec T11 checks the wrong thing |
| 6 | hole identity by signature | FAILS |
| 7 | Prop. 3.7, confluence under R5 | confluence HOLDS per path; R3/R5 break addressing: FAILS |
| 8 | Prop. 2.2, the quotient is a ruler | HOLDS, and it undercuts "dark never includes a ruler"; repair given |
| 9 | Lemmas 7.7-7.9 | 7.7 HOLDS, 7.8 HOLDS, 7.9 HOLDS-WITH-REPAIR; §7.4 item 1 ("only L₀+L₁ carries b₁") FAILS |
| 10 | the four addresses | they don't collapse; two leaks into dark, one non-homomorphism |

---

## §1 Prop. 7.2: prism's `Oid::hash`

### What the code does (read, then reproduced bit for bit)

`Oid::hash` (`oid.rs:28`) calls `coincidence::canonical_hash` (`coincidence.rs:480`):

1. `encode_into_basis` (`:207`): for each byte position `i`, 1 + 16 SHA-256 calls produce a vector `e(i, xᵢ) ∈ [-1,1]¹⁶`; `s = Σᵢ e(i, xᵢ)`, summed sequentially per coordinate.
2. Three `Projection`s, each `rank1` (`:153`): `P = n nᵀ` with `n` the normalized seed vector, entries `|nᵣn꜀| ≤ ε` dropped.
3. `apply` (`:183`) computes each output coordinate as `result[row] += P[row,col] * s[col]`, iterating a `BTreeMap<(String,String),f64>`, i.e. in **lexicographic** label order (`d0, d1, d10, …, d15, d2, …, d9`).
4. SHA-256 over `"prism-core:coincidence:" ‖ "coincidence:" ‖ 3u64 ‖` the **3 × 16 = 48** output floats.

My reimplementation (`oid_seam.py`) matches the Rust oracle (`oidref/`, a 10-line binary calling `prismqueer::Oid::hash` at `7e6b00f`) on **170/170** inputs (the pin `b"prism"`, the empty input, and 168 random inputs of lengths 1–39, 100, 257, 1000).

### Finding 1a: the digest is not a function of the three scalars (proof step FAILS)

Math Prop. 7.2: "Any two inputs with bit-identical triples `(⟨v₁,s⟩, ⟨v₂,s⟩, ⟨v₃,s⟩)` get the same `Oid`." The code never computes `⟨v,s⟩`. It computes 48 separately rounded sums `Σ_c fl(nᵣn_c)·s_c`, each depending on all 16 coordinates of `s`. Rank one holds in exact arithmetic, not in the hashed bytes.

*Reproduction* (`oid_triple.py`): take `s = encode(b"prism")`. Build `s' = s + εd` with `d` an exact rational null vector of the three `n_k` (ε = ±2⁻³⁰…2⁻⁴³), rounded to f64. Keep the states whose three exact dot products round to the same f64 as `s`'s. Result: **121 states with a bit-identical triple; 0 share the digest**; 23–26 of the 48 hashed floats differ in each.

So "collision resistance bounded by three float sums, at most 192 bits of state" is the wrong model. The input passes through three reals (that part is right), and then through ~48 roundings that don't factor through them.

### Finding 1b: the CONJECTURE "collisions far below 2¹²⁸" is UNTESTED, and I estimate against it

What a collision needs: either `s(x) = s(x')` in all 16 floats (≈ 16 × 53 bits, nothing in the structure helps), or a near-collision of the three projections to about 1 ulp **and** agreement of all 48 roundings. Wagner's k-tree applies to the first half (the projections are sums of per-position terms). Finding 1a measured the second half: about half the 48 roundings disagree even for perturbations of 2⁻³⁰ to 2⁻⁴⁴ relative, so luck costs on the order of 2⁴⁸ or more per near-collision. On top of that, the sequential float sum of `s` has rounding noise around 10⁻¹² for kilobyte inputs, which is much larger than the ulp target, so the linear structure k-tree needs isn't exact in the computed `s`. I did not build a collision, and I don't believe one is cheap. **The structure leaks under SHA-256; I found no way to exploit it.**

That changes the headline: prism's content addressing is not *broken* content addressing. It is content addressing **with no security argument**: no reduction to SHA-256, collision resistance resting on unanalysed float noise. That's enough to keep it out of the root address, and it is a different sentence to say to prism-Reed.

### Finding 1c: the digest is defined by `BTreeMap` iteration order, not only by summation order (sharpens Mara)

Mara found that Python's compensated `sum()` breaks the pin. Same order, different association: also broken (`math.fsum` port: pin mismatch). And a port that keeps sequential summation but iterates labels **numerically** (`d0 … d15`, what any reader of the docblock would write) also misses the pin: `Oid::hash(b"prism")` becomes `96c045951648ef5e…` instead of `08f8e91d…` (`python3 oid_seam.py`, line 2). The specification of prism's identity function is the lexicographic sort order of the strings `"d0"…"d15"` in a Rust `BTreeMap`. No document states it. The BEAM port in scout-spectral-uuid and any mirror-in-mirror engine have to replicate it by reading Rust.

### Finding 1d: cost (new)

17 SHA-256 calls per input byte. Oracle on a 200 KB input: 0.98 s (`/usr/bin/time -p oidref < big.hex`), i.e. about 200 KB/s, about 5 µs per byte. A compiler that hash-conses every splinter of every shard can't route through this.

### Verdict

**HOLDS-WITH-REPAIR.** The conclusion and Decision 5 stand: `addr` is BLAKE3 over the canonical encoding, and `Oid::hash` is not the identity. Repair the stated reason: replace "a function of three f64 scalars, at most 192 bits" with "a function of 48 floats that depend on all 16 coordinates of `s`; no reduction to SHA-256 exists; the value is fixed by IEEE-754 operation order *and* `BTreeMap` label order; 17 SHA-256 per byte". Withdraw the cheap-collision CONJECTURE, or keep it labelled as open with Finding 1b's estimate next to it.

---

## §2 Prop. 1.7: the tick out of the address

**The proposition HOLDS** as stated: an inflationary set iteration whose new elements carry a strictly increasing tick never stabilises while `step` is non-empty. The proof is three lines and correct.

**The step from it to T15 FAILS for `rotation.rs`.** Moving `tick` into a deposit record is necessary but not sufficient, for two reasons.

*Finding 2a: the content never repeats.* The splinter content is `(phases, order_r, fiedler)` (`rotation.rs:264-278`). `native/spectral.f90:188-210` integrates `θ ← θ + dt·(ω + K/N Σ sin(θⱼ−θᵢ))` with **no wrapping**. Summed over `i` the coupling terms cancel, so the mean phase moves by `dt·mean(ω)` per tick, and `ωᵢ = 0.3(0.1i + 1) > 0` (`rotation.rs` init). In exact arithmetic the absolute phases never repeat. *Reproduction* (`kuramoto_seam.py`, same Euler scheme and initial conditions, n = 3, 5, 8, 20 000 ticks): no absolute state repeats. With `tick` removed, T15 ("two ticks that observe the same state deposit one splinter twice") still never fires.

The repair is Alex's own sentence: **rotation is free.** A common phase shift is a symmetry of the Kuramoto system, so the splinter's content must be the quotient by it (phases relative to one oscillator, or with the mean subtracted). In the same simulation the gauge-fixed state becomes **bit-identical at consecutive ticks** at tick 1347 (n=3), 1372 (n=5) and 767 (n=8). (Python's `sin` is not gfortran's, so the tick numbers are illustrative; the fact that the absolute state never repeats is not.) So the address should be `(relative phases, r, λ₂)` deposited at `tick`. That's the free move applied before hashing, which is exactly what §2 of the math asks for everywhere else.

*Finding 2b: "nothing new falls" is the wrong stop test for non-monotone steps.* With `tick` out, a period-2 orbit `A, B, A, B` gives `S = {A, B}` after two ticks, and the inflationary test `S_{t+1} = S_t` reports a **fixed point** on an oscillation. Math §1.5 says "if two ticks observe the same state, that is one splinter deposited twice. That's the property a fixed point needs." That's insufficient: the same splinter deposited at *non-consecutive* ticks is a cycle. For Datalog (monotone, §5) the cumulative test is right. For rotation, and for the stage iteration of §6.3, the test must be consecutive equality, `addr(state_{t+1}) = addr(state_t)`, with `cycle(p)` reported otherwise. Spec §3 already does this for `--fixpoint`; spec §4 ("one rotation core") must not carry the cumulative test over to `rotation.rs`.

**Repair for T15:** "Two *consecutive* ticks whose states agree *modulo a common phase rotation* deposit one splinter twice; the same splinter deposited at ticks `t` and `t+p`, `p > 1`, with different splinters between, is reported as `cycle(p)`, never as fixed."

---

## §3 Prop. 4.1: choice needs `split`; and `project` is not lossy

### The `project` correction: HOLDS

prism `docs/loop/CURRENT.md` §7b″ (Decided 2026-10-27): "`project` is emission … append-only. It is not a spectral band projector." The pinned `lib.rs:202` still says "`project` — the lossy transformation." Drift on prism's side, as Mara says (spec §12 item 12).

One caveat that the math doesn't draw. §7b″ also says `project` is "the one op with an **effect**: it writes (labels, light, presence)". Injective, yes. Free in the sense of math §2.1 (`μ ∘ g = μ`), no: it *adds* meaning, namely the rule-name labels that math §1.4 says *are* meaning. See §8 below: the math uses "free" for two different things.

### Prop. 4.1 itself: HOLDS-WITH-REPAIR

The proof needs every letter of the word to be dark-absorbing. It cites the Beam contract for that. But the contract (`beam.rs:33`) binds `tick`, `smap` and `next`, and says nothing about `Prism::focus/project/settle` (`lib.rs:190-206`). Those take a whole beam and may read `result().err()`. A `settle` that maps `Failure(e)` to light is legal under the trait.

Second, the error type is generic. In a PEG over a fixed input string the only state is a position, and failure at position `i` naturally carries `i`: Mara's own Datalog writes `f(e, i)`. So the original input is *not* lost when `e₁` goes dark. It's in the failure.

*Counterexample to the proof as written* (a three-letter encoding with no copy of the input):

```
e₁ / e₂  =  w(e₁) · settle_recover · lift(w(e₂))
settle_recover : Success(j) ↦ Done(j),  Failure(i) ↦ Todo(i)
lift(w)        : the same word with each prism lifted to pass Done(j) through unchanged
```

It needs neither a fork nor a hidden branch inside one op's body. What it does need is (a) a non-dark-absorbing `settle`, and (b) every letter of `w(e₂)` dispatching on a sum type (`Done | Todo`). (b) is what an optic *prism* is (preview on one constructor), so "the branch is hidden in an op body" is true only in the sense that every lifted letter carries a case split.

**Repair.** State the hypothesis as "`settle` is dark-absorbing on unary words", which the contract doesn't give and the math's own `!e` row contradicts (settle "keeping the failure branch" turns dark into light). State the conclusion as: **choice needs a coproduct eliminator: either a binary join, or a case split carried by the letters.** `split`+`settle` is one realisation. It's the right one for mirror if Decision 8 lands the trace DAG's Fracture/Join, because then the choice is visible in the trace. But it's a design choice, not a theorem. Also note that `split` in prism's sense (Divisi: both branches *run*) is semantically fine for a pure PEG but evaluates `e₂` even when `e₁` succeeds. Packrat memo and bottom-up evaluation hide that cost; a lazy evaluator wouldn't.

---

## §4 Prop. 5.1: a PEG is positive Datalog

### Proof (replaces "random testing")

Ford 2004 defines `⇒` by one inference rule per expression form. The Horn clauses of math §5.1 transcribe those rules over positions, so the least model is the set of judgments with a **finite derivation**. That half is Mara's proof and it is correct. What was missing is the packrat side, and it's short:

*Packrat's evaluation order follows the necessary premise.* In every rule the result of `(e, i)` is determined only after `(e', i)` is known for the first sub-expression `e'` (sequence: `e₁` at `i`; choice: `e₁` at `i`; `e*`, `!e`, `&e`: `e` at `i`; reference: the body at `i`). A packrat interpreter evaluates exactly that premise first. So if packrat re-enters `(e, i)` while computing it, every derivation of `(e, i)` contains a derivation of `(e, i)` as a proper sub-derivation, and none is finite: the atom is undefined. Conversely, if packrat terminates, its call tree *is* a finite derivation. By Ford's functionality (`⇒` is a function, for every grammar) there is at most one. **Packrat loops on `(e, i)` ⇔ `(e, i)` is undefined in the least model.** That's the 520/520 Mara observed, as a theorem.

### Exhaustive check (COMPUTED, not sampled)

`peg_seam.py`: my own top-down evaluator with loop detection and my own naive bottom-up least model of Mara's clauses (plus `&e`, which spec §2 item 3 uses and the math's table omits: `m(&e,i,i) :- m(e,i,j). f(&e,i) :- f(e,i).`). Every pair of rule bodies `A, B` over `{ε, 'a', 'b', A, B, seq, alt, *, !, &}` up to 3 nodes, every input over `{a,b}` up to length 3, **every sub-expression at every position**, not only the start atom:

```
SIZE<=3 LMAX<=3: grammars=13225 atoms=3933328 loop/undefined atoms=1292316
C1 functionality violations: 0
C2 TD/BU disagreements: 0
```
```
SIZE<=4 LMAX<=2: grammars=490000 atoms=64334120 loop/undefined atoms=23501435
C1 functionality violations: 0
C2 TD/BU disagreements: 0
```

**Verdict: HOLDS.** Small repairs: add `&e`; say that `f("c",i) :- ¬input(i,"c")` is negation on the EDB only (stratum 0), so "positive" means "positive over the input's complement"; the `j > i` guard on `e*` is redundant against Ford (a nullable `e` under `*` has no finite derivation either way) but harmless.

### Spec repair: what `undefined` counts

Bottom-up evaluation derives **every** `(e, i)`, including atoms the start symbol never needs. Spec §1.1 counts all of them, and any undefined atom makes the verdict `partial`.

*Reproduction* (`undefined_scope.py`): `S <- 'a' / L ; L <- L 'b'` on input `a`. Start atom `m(S,0,1)` is derived and packrat succeeds, but the least model has **6 undefined atoms** (all `L`'s). The spec would report `partial` for a successful parse of a non-well-formed but harmless grammar.

**Repair:** count only undefined atoms in the dependency cone of the start atom (the atoms its derivation attempt needs), and report the grammar's `WF` failure separately (T6a already checks it statically).

---

## §5 Prop. 6.2: the fixed point can't certify an honest seed

The proposition (uniqueness can fail) is true. The **proof puts the trojan in the wrong place**, and that moves a test.

Thompson's trojan lives in the compiler **binary**. In mirror's stages the binary is the engine `E` (and the shared `strip`/`nf`, and the floor that runs them), not the seed's output `N₀`. Mara's scenario is a seed that misreads, with `N₀` encoding the misreading so that `E(N₀)` reproduces it. That is mostly *caught* by the k = 1 equation, as long as `E`'s transcription is faithful (leaves = input bytes, labels = rule names, children in order). The reason: `T(N) = strip(E(N)(G))` pins every leaf of `T(N)` to `G`'s bytes. A misread `N₀` survives only if it is itself the transcription of `G` under the grammar it denotes.

*Reproduction* (`fixpoint_seam.py`): a miniature of spec §3. It has a 22-rule self-describing PEG `G` (Ford-style), a hand-written recursive-descent seed `P₀`, an engine `E` interpreting the grammar AST, `strip` with trivia rule `Sp`, and SHA-256 over the canonical tree.

```
X1 honest: addr(N0) = 0f909f9587b75a0d  stage-2 fixed point: True
X2 seed misreadings: 324 distinct single-site mutations of N0; 0 pass the k=1 equation
X3 E_long: stage-2 fixed point: True; DDC addr(N0) equal to honest: True
X3 E_troj: stage-2 fixed point: True; DDC addr(N0) equal to honest: True
   user grammar U = "Top <- S !. / S <- 'a' / 'ab'" on input 'ab': honest -> False | E_long -> True
   user grammar with rule Admin, rule Guard <- !'root' Name, on input 'root': honest -> False | E_troj -> True
```

- **X2.** Every single-site seed misreading I could generate fails stage 2. The mutations were: swapping alternatives, reordering sequence items or rules, changing `* + ?`, dropping a rule, dropping a `!`. This is COMPUTED on single-site mutations, not a proof for all misreadings.
- **X3, the test vectors Mara asked for.** `E_long` reads ordered choice as longest match. That's an *honest* bug, and it's exactly the PEG shear of math Prop. 2.4. `E_troj` reads `!e` as `&e` whenever the grammar being run defines a rule named `Admin`. **Both pass stage 2 and both pass spec T11** (the second witness is a seed; it is independent of `E`, so `addr(N₀)` agrees). Both misread a user grammar. `G` exercises no choice where first-match and longest-match differ, and has no rule `Admin`, so the self-reading can't see either.

**Consequences.**
1. Spec T11 (DDC on `addr(N₀)`) checks the seed. That's the part the fixed point already checks. Wheeler's DDC compiles "the putative compiler source code" (Wheeler 2009/2010, https://arxiv.org/abs/1004.5534); here the putative compiler is `E`. **Repair T11:** if `E` is a mirror shard (Alex: the evaluator lives in mirror), DDC applies to `E`'s source: build `E` from its shard with two independent floors and require bit-identical `E` artefacts. Add a **semantics oracle independent of `G`**: a corpus of user grammars and inputs with known verdicts (Ford's examples, Prop. 2.4's shear, `!`/`&` vectors) run through `E(N₀)`. `G`'s self-reading can never test a construct `G` doesn't exercise on itself.
2. Shared `strip`/`nf` is in the same blind spot. A normalizer that sorted choice children (prism's `MerkleTree` "sorted by Oid", math §1.3) would pass T1 everywhere. Only T2's alternative swap catches it, so T2 must be a gate on T1's green, not a follow-up.
3. `--fixpoint` with `k_max ≥ 2` launders seed misreadings: a mutated `N₀` goes to `N_true` in one step and then stays there. Report `fixed at k` and treat `k > 1` as a failed stage 2, as spec §3 already requires for `k = 1`.

---

## §6 Hole identity: addressing `\` by its signature — FAILS

Lean (math §3.4, spec §5, Decision 4): a hole is addressed by `hole(signature)`, so "two holes asking the same question share an address, so one answer serves both".

**Counterexample 6a (same signature, different questions).**
```
io normalize(x: path) -> path { \ }
io parent(x: path)    -> path { \ }
```
Both holes get `hole(path → path)`, which is one address. In `N` they hash-cons to **one** splinter. A resolution recorded against that address is a resolution of both. In the rerere picture Alex gave, git's rerere *keys* a resolution by the conflict's content but *applies* it at each occurrence. Here the key and the occurrence have been made the same thing. Offering `parent`'s body for `normalize` is fine, because offers are what the arc is for. But *identity* says they are the same hole, and `Γ` gains a reuse edge (`b₁ + 1`) between two unrelated functions.

**Counterexample 6b (no signature).** Spec §2 item 9 makes `\` valid "wherever a body **or value** is expected". A value hole with no declared type (`x = \`, a record field, a `requires \`) has signature `?`. Every such hole in the system gets the same address.

**Counterexample 6c (the signature isn't the question).** The question a hole asks is the judgment `Γ ⊢ ? : T`: the bound variables in scope with their types, the goal, and the pacts (`requires` / `ensures`) in force. `f(x: a) -> b` names only the parameters and the result. Two holes under the same signature with different `requires` clauses ask different questions.

**Mara's argument against path addressing doesn't apply.** Prop. 1.7 forbids coordinates that change under iteration (ticks) or under free moves (source spans, E3). A path **in `N`** is invariant under every free move, because `N` is the quotient. It's a coordinate of the occurrence, not of the source.

**Repair.** Two identifiers:
- *question* = `H("hole\0" ‖ τ ‖ enc(Γ ⊢ T) ‖ enc(pacts in scope))`, with de Bruijn-normalised context. Same question, same address. Resolutions are **offered** by this key.
- *occurrence* = `(addr(N), path in N)`, a deposit coordinate. Resolutions are **applied** by this key. `mirror fate` takes an occurrence.

T7 then reads: two `\` with the same judgment share a *question* address and have two *occurrences*; `holes` counts occurrences.

---

## §7 Prop. 3.7: confluence of R1–R4 under R5 — confluence HOLDS per path; path dependence FAILS addressing

R1–R4 (2026-08-10 §1.2) each delete one top-level item of a composition `X ∘ M`. Their side conditions read the item itself and the file path, never another item. Deleting two different items commutes, so there are no critical pairs **for a fixed path**, and Newman's lemma gives unique normal forms per path. (UNTESTED caveat: if the landed P1 detector checks the prism's `@X` against the shard's *declared* `out`, then R3 deleting that `out` changes whether R1 fires, and there is a critical pair. I didn't read `apply_h.rs`; T8 should include exactly this pair.)

The problem is not confluence. R3, R4 and R5 make the normal form depend on **the file path**, which is a source coordinate. That's what Prop. 1.7's discipline keeps out of identity, and it breaks Prop. 2.1 in both directions.

- **7a (same bytes, different address).** Put the same bytes, containing an identity prism, at `shards/prism.mirror` and at `shards/x/prism.mirror`. R5 exempts the first only, so the normal forms differ and the roots differ. Same holds for `out @a` at `shards/a.mirror` (elided by R3) and at `shards/b.mirror` (kept).
- **7b (same address, different meaning).** A shard with **no** `out` at `shards/a.mirror` and at `shards/b.mirror`. Both normal forms lack `out`, so the roots are equal. The exports differ (`τ_path` gives `@a` and `@b`). Equal `addr`, different meaning.
- **7c (the spec contradicts itself).** Spec §2 item 1 requires `out @mirror` in `mirror.mirror`, and §2 item 6 puts R1–R4 with R5 into `μ`. T1.3 compiles `tests/fixpoint/mirror.reformatted.mirror`, a different path, and requires the same root. If `τ_path(mirror.mirror) = @mirror`, R3 elides `out @mirror` in the original and keeps it in the copy, and **T1.3 fails by construction**. If `τ_path` is undefined at the repo root, R3 never fires there and T1.3 passes, but then every `shards/` file has the 7a/7b problem.

**Repair.** Normalize toward *explicit*. Reverse R3: insert `out τ_path(path)` when it is absent, and keep it when present. Path then enters `N` only to supply a missing export, which is meaning the file really does take from its location. Replace R5's path test with a content test: the self-declaring shards are recognised by what they declare, not where they sit. Or drop R5 and let their identity prisms elide like everyone else's. R4 (docblock template) becomes moot if Decision 2 makes comments trivia.

---

## §8 Prop. 2.2: "which moves are free is itself a ruler" — HOLDS, with two consequences

The linear algebra is correct (isometry is relative to `M`).

**8a. "Free" has two definitions in the math.** §2.1: free means `μ ∘ g = μ` (meaning-invariant). §2.3: free means injective (isometric under the discrete metric). They disagree:

| move | injective? | `μ`-invariant? | math says |
|---|---|---|---|
| renaming an exported rule | yes (bijective) | no | ruler (§2.5) |
| `project` (emission) | yes | no: it adds labels, and labels are meaning | free (§4.4) |

**Repair:** three classes, not two. *Free* (`μ`-invariant): no identity change, no ruler. *Edit* (injective, meaning-changing): identity changes, no ruler needed. *Ruler-carrying* (non-injective): needs a declared ruler. Prop. 4.2 ("every loss of meaning happens at a `settle`") is about the third class and survives.

**8b. Identity is relative to `μ`, and `μ` is named by a name.** Any OID is `addr ∘ c_μ`. The splinter carries `τ = @mirror`, a name, not `μ`'s address. If the `@mirror` grammar changes `μ` (say comments become meaning), old and new addresses live in one namespace, and an equal `addr` across the change asserts that two sources are the same under two different quotients. So whose declaration is it? Whoever controls the name.

You can't fix this by putting `addr(μ)` into every `τ` all the way down. For `mirror.mirror`, `μ` *is* the file, so its own root would have to contain its own root, and a hash has no such fixed point. **Repair:** every format other than the self-describing root carries `τ = (name, addr(format grammar))`. The root carries a name, and that name is a ruler signed by its owner (Alex). Write it down as the one irreducible signature, next to the floor of math §6.1.

**8c. Spec §10 contradicts itself.** "Dark never includes a ruler" is followed by "the result of a ruler-carrying op carries the ruler's address in its own content", which puts the ruler's address into dark. PEG alternative order (a ruler per §2.5) is in dark too, and so is `μ` (8b). **Repair:** "dark never includes an *unnamed* ruler."

(Mara's §11 item 6, a free move `μ` can't declare: any equivalence without a computable canonical form, e.g. language equivalence of grammars, which is undecidable for CFGs, or βη-equality of untyped terms. So the OID is the quotient by the *declared, decidable* free moves. That's worth one sentence in §2.)

---

## §9 Lemmas 7.7–7.9, §7.4 item 1, Prop. 7.11

COMPUTED with `spectra_seam.py` (numpy, my own DAG generator with real hash-consing).

| claim | result | verdict |
|---|---|---|
| 7.7: `b₁ = \|E\|−\|V\|+1 = Σ_{v≠root}(indeg−1)` | 500/500 random hash-consed DAGs | HOLDS |
| 7.8: `A` nilpotent; `spec(D_out − A)` = out-degrees | 300/300 | HOLDS |
| 7.9: uniform phase sees only unequal lengths | diamond (2,2) invisible; triangle (1,2) visible | HOLDS-WITH-REPAIR |
| §7.4 item 1: "only `L₀ + L₁` carries `b₁`" | `b₁ = tr(L₀)/2 − n + mult₀(L₀)`: 500/500 | **FAILS** |
| 7.11 for the Laplacian | none ≤ 10 vertices; a pair at 11 | conclusion HOLDS; Mara's "8" is adjacency |

**7.9 repair.** The flux around *any* cycle of the underlying graph is `θ·(#forward − #backward)`, not only around two-path reuse loops. Invisible iff `θ(f − b) ∈ 2πℤ`:
- a zigzag `a→c, b→c, b→d, a→d` (net 0) is invisible;
- at `θ = π/2`, a net-4 loop (direct edge plus a path of 5) is invisible;
- `f(x, x)` (two parallel edges) is invisible for every `θ`.

So the magnetic operator never sees the most common reuse in an AST.

**§7.4 item 1 counterexample.** `L₀`'s spectrum alone determines `n`, `b₀` (zeros) and `|E|` (trace/2), hence `b₁`. `L₁ = BᵀB` has exactly `L₀`'s nonzero eigenvalues plus `b₁` zeros (500/500). `L₀ + L₁` adds **no** information at the spectrum level. It changes only which compressed *features* expose `b₁`: heat trace `h_{L₀+L₁}(t) = 2h₀(t) − b₀ + b₁`. **Repair for Decision 10:** don't choose an operator to smuggle an integer through a float. Put `b₁` (exact, integer, from Lemma 7.7) into the coordinate as its own field, and spend the float axes on what isn't already exact.

**Does `active` survive free moves once the builder is real?** Yes, provided `Γ` is built from `N`. Then `Γ` is a function of `N`, and Prop. 7.10 covers relabelling. Two notes. `Γ` ignores child order, so a ruler move (swapping alternatives) leaves `active` unchanged: it locates, as Mara says. And with de Bruijn indices and hash-consing, `Var(0)` and common literals become hubs shared by every rule, so `b₁` and the low spectrum mostly count recurrences of trivial atoms. That's a quality issue for a locator, not a correctness issue; T9 should include a fixture showing it.

---

## §10 The four addresses (Reed's item)

**They don't collapse.** `addr` and the source lens differ by prefix (`"fragment\0"` vs `"lens\0"`). Git ids carry git's own header. `uuid` is a different width. With `active = 0`, `uuid` is a pure function of `dark` and carries nothing extra, which is honest, since spec §8 prints `active unsigned`.

**Leaks and breaks:**
1. **`μ` in dark** (§8b): dark is computed under a quotient named only by `@mirror`.
2. **`SpectralUuid` isn't an RFC 9562 UUID.** `from_parts` (`spectral_uuid.rs:93-101`) writes dark into bytes 6–15 and sets no version or variant bits. RFC 9562 puts the version in octet 6 and the variant in "the most significant bits of octet 8" (https://www.rfc-editor.org/info/rfc9562/), and both are dark bits here. Any library that validates or re-stamps v8 UUIDs on the way through overwrites up to 6 of the 80 dark bits. **Repair:** either don't print it in 8-4-4-4-12 form, or spend 6 bits on version/variant and say dark is 74 bits.
3. **`SpectralUuid::EMPTY` is unreachable from `compile`.** Its dark is the first 10 bytes of `BLAKE3("")` (`:77-82`). Under math §1.2 the empty shard's address is `H("fragment\0" ‖ τ ‖ …)`, not `BLAKE3("")`. The lattice bottom that the CRDT spec and `spectral_uuid.rs:58-63` rely on is not a value compile produces. **Repair:** define `EMPTY` as `uuid(compile(empty shard))`.
4. **"Only dark's Merkle rule composes" (math §7.4 item 5) is false at uuid altitude.** `dark(a ⊔ b)` is a prefix of a hash of the *full* child addresses, and it can't be computed from `dark(a)` and `dark(b)`, which are 80-bit truncations. The **address** composes; the uuid is a homomorphism in neither half. **Repair:** store and merge on `addr` (256 bits); derive the uuid at the edge.
5. **Git side.** Prop. 7.4's isomorphism holds "barring a collision in either hash", and a default repo uses SHA-1 (git's own hash-function-transition doc treats it as no longer collision-resistant). T1.5's re-hash walk is what makes `refs/mirror/root/<addr>` trustworthy. **Repair:** make it the read path (re-hash on every load from git), not only a test.

---

## Which §12 decisions change

| # | decision | change |
|---|---|---|
| 4 | holes by signature | **reverse**: question = judgment `Γ ⊢ T` + pacts (offered by it); occurrence = `(addr(N), path in N)` (applied by it). §6 |
| 5 | BLAKE3; `Oid::hash` a measurement | **stands**; replace the stated reason (§1) |
| 7 | move `tick` out of `rotation::Splinter` | **insufficient**: also quotient the common phase rotation; stop test = consecutive equality, cycles reported. §2 |
| 8 | `split` as Fracture | **reframe**: what is needed is a coproduct eliminator; Fracture/Join is one choice. §3 |
| 9 | where the seed lives | **demoted**: the trust root is `E` and the floor. New decision: DDC on `E`'s source plus a `G`-independent semantics oracle. §5 |
| 10 | `L₀ + L₁` vs `L₀` | **argument void**: `L₀` already carries `b₁`; put `b₁` in the coordinate explicitly. §9 |
| 11 | `active` `@ruler` | add: uuid version bits, `EMPTY` from compile, merge on `addr` not uuid. §10 |
| new | R3 / R5 | reverse R3 (insert, don't elide); R5 by content, not path. §7 |
| new | `τ` | `(name, addr(format grammar))`; the root's name is the one signed ruler. §8 |
| new | `undefined` | count only the start atom's dependency cone. §4 |

Decisions 1, 2, 3, 6 and 12 are unaffected (6 gains the re-hash-on-read note).

## Against myself

- §1b is an estimate, not a bound. I didn't attempt the k-tree. Someone with more time may find the rounding noise more correlated than my 121-sample measurement suggests.
- §5 X2 tests single-site mutations only. "Every seed misreading is caught" is not proved, only "none of 324 survived".
- §9: the Laplacian-cospectral tree search is random (40 000 samples per `n`), not exhaustive.
- §4: the exhaustive runs are bounded (rule bodies of ≤ 4 nodes, inputs of length ≤ 3 and ≤ 2). The proof carries the general case. The runs check my transcription of it.
