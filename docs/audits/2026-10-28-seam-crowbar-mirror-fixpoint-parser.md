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
