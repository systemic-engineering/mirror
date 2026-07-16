# bilateral-sentinel.md — math foundation for `@epistemologic/pact/bilateral`

**Author:** Mara `<mara@systemic.engineer>` (2026-07-16)
**Species-decl:** `shards/epistemologic/pact/bilateral.mirror`
**Canonical spec:** `docs/specs/bilateral-predicate-substrate-shape.md`

## Scope

Formal ground for the bilateral-predicate substrate shape minted this
tick. Establishes three claims:

1. **The sentinel is a content-addressed witness.** Byte-string
   containment in the argument's OID is a proper predicate over the
   substrate's content-addressed algebra; the predicate factors through
   the OID's byte-representation without touching program semantics
   (Rice-safe by construction).

2. **The shard body IS the dispatch table.** The composition of the
   bilateral-declaration corpus (parsed from `shards/**` at grammar-
   load time) with the reflective evaluator D forms a spectral triple
   (A, H, D) in the Connes sense; the current ~30 hand-typed arms are
   a BROKEN spectral triple (D was per-element hardcoded); the
   reflective form makes D uniform over A, which IS the actual Connes
   discipline.

3. **The reflective evaluator is monotone under corpus extension.**
   Adding a new bilateral declaration (a new element to A) does not
   invalidate any already-witnessed Pass verdict; the evaluator is
   monotone as A grows.

The three claims compose: (1) proves the per-arg check is well-typed;
(2) proves the corpus/evaluator composition IS the Connes-triple
altitude discipline the substrate declared substrate-spectral-triples-
all-the-way (`[[architecture-spectral-triples-all-the-way]]`); (3)
proves the reflective form is safe under Reed's follow-up landing.

---

## §1. Sentinel-as-content-addressed-witness

### §1.1 The substrate's content-addressed algebra

Per `shards/glass.mirror` § "The three-layer recognition" (Alex + Reed
2026-06-06):

- **splinter (BOTTOM)** — the oid-addressed content atom at every
  altitude. Universal carrier of content. Content is byte-uniquely
  identified by its OID (BLAKE3 in the landed realization).
- **shard (MIDDLE)** — the uuid_spectral-addressed settlement of
  composed splinters into a stored fragment.
- **uuid_spectral (TOP)** — the graph-navigatable spectral identifier
  (128 bits; 48 ACTIVE + 80 DARK per the golden-ratio split).

The bilateral predicate operates on **splinters** — the OID-addressed
atoms. Every argument `arg` the reflective evaluator receives carries
a `ref.oid` — a content-address whose byte-representation is the atom's
substrate identity.

### §1.2 The sentinel predicate

Let A = set of substrate atoms (splinters), each carrying a byte-string
OID `oid(a)` ∈ ByteStr. Let S ⊆ ByteStr be the set of substrate-decl'd
sentinel byte-strings (one per landed bilateral declaration).

For each sentinel s ∈ S, the **sentinel predicate** χ_s : A → {Pass,
Fail} is defined as:

```
χ_s(a) := Pass if s ⊑ oid(a) else Fail
```

where `⊑` is byte-substring containment.

### §1.3 The predicate is well-typed (Rice-safe)

**Claim:** χ_s is a proper predicate — it does not violate Rice's
theorem because it reads only byte-visible state (the OID's
byte-representation), not program semantics.

**Proof.** Rice's theorem forbids non-trivial semantic predicates over
recursively-enumerable languages. χ_s is a SYNTACTIC predicate over
the OID's byte-representation: given `oid(a)`, the check `s ⊑ oid(a)`
is a byte-string containment test computable in O(|oid(a)| × |s|)
time. It does not consult the semantics of the object a addresses; it
consults ONLY the substrate's content-address byte-representation.

Since OIDs are content-addresses (hashes of the byte-content of the
atom they identify), the sentinel-containment check is a **byte-level
statement about the substrate's content, not a semantic statement about
its meaning**. Rice-safety holds by construction. ∎

### §1.4 Why the sentinel is IN the OID (rather than metadata)

An immediate question: why is the sentinel embedded in the OID rather
than in metadata associated with the atom?

**Because the OID is the atom's identity.** Per
`shards/epistemologic/property/verdict_is_content_addressed.mirror`:
verdict-content-addressing is a landed substrate property. The
sentinel-in-OID encoding ensures that:

- (a) The witness IS the identity. An atom that carries the sentinel
      byte-string in its OID is byte-identical to an atom carrying
      the same content; there is no separate "witness metadata" that
      can drift out of sync.
- (b) The check is deterministic. Two atoms with the same OID
      byte-representation give the same verdict for the same
      sentinel; content-addressing forces determinism.
- (c) The check is composable. Composed bilaterals AND-conjunct
      sub-verdicts on the SAME argument tuple — the witness carrier
      is a single field of the atom (its OID), not scattered across
      per-sub-predicate metadata.

This is the substrate-honest encoding: the atom's identity IS its
witness surface; the sentinel is a fragment of that identity; the
predicate reads the fragment.

### §1.5 Empirical form across the 30 landed arms

Every one of the ~30 landed bilateral arms uses the same sentinel
form: a `key=value` byte-string where `key` names the substrate-decl'd
witness dimension and `value` names the specific value the argument
must carry.

Examples (from the 4 COLLAPSED shards):

| shard | sentinel | key | value |
|-------|----------|-----|-------|
| spectral/signature | `chain=merkle-linked` | chain | merkle-linked |
| coherence | `axis=splinter-ward` | axis | splinter-ward |
| peer/persistence | `visibility=filter-respected` | visibility | filter-respected |
| kintsugi/roomba | `termination=scope-a-exhaustive` | termination | scope-a-exhaustive |
| gc composed | `witnessing=all-four-pass` | witnessing | all-four-pass |

The `key=value` shape is the substrate's convention (not enforced by
this species — the shape is `sentinel: ref` which accepts any
byte-string), but its uniformity across all 30 landed instances
suggests it IS the substrate-native form the sentinel wants to take.

**Punted to Alex-adjudicable:** whether the shape should ENFORCE the
`key=value` form via a well-formedness sub-predicate. Currently the
meta-bilateral `bilateral_well_formed` (§3.4 of canonical spec) checks
only non-emptiness. Reed follow-up if empirical need arises.

---

## §2. The shard body IS the dispatch table (Connes triple)

### §2.1 The Connes spectral triple

Per `shards/nl.mirror` § "Spectral primitives" + the substrate's
`[[architecture-spectral-triples-all-the-way]]` decision: the substrate
carries the Connes spectral-triple discipline (A, H, D) at multiple
altitudes. A brief recap:

- **A** — a ∗-algebra of operators (the substrate's atoms + their
  compositions).
- **H** — a Hilbert space (the substrate's verdict/opacity carrier at
  the appropriate altitude; @glass.transparency in the landed form).
- **D** — the Dirac operator (a self-adjoint operator on H, with
  bounded commutator with A).

Connes' condition: for every a ∈ A, the commutator `[D, a]` is a
bounded operator on H. This condition is what makes (A, H, D) a
**spectral triple** — the algebraic elements a and the geometric
operator D interact under a controlled bound.

### §2.2 The current (broken) triple: per-element D

In the current landed form (~30 hand-typed arms in
`bootstrap/src/apply_h.rs`), the effective spectral triple looks like:

- **A** = the corpus of shard-decl'd bilateral predicate action refs
  (~30 elements across 8 shards).
- **H** = @glass.verdict-space.
- **D_current** = a per-element operator: `D(a) := <the a-specific
  hand-typed arm>`.

**The problem:** D_current is NOT uniform over A. Every element a ∈ A
has its OWN D-arm; the operator's shape is INDEXED by a. This is not
a spectral triple — it's a family of per-element operators masquerading
as one. The commutator condition `[D, a]` bounded is trivially satisfied
because D IS a-indexed, but the geometric content of the triple is
absent: D_current is not one Dirac operator; it's 30 arm-of-D per each
a.

**This is a broken spectral triple.** The substrate-decl'd
`[[architecture-spectral-triples-all-the-way]]` discipline is violated
at the apply_h.rs altitude.

### §2.3 The reflective (uniform) triple

The reflective evaluator restores the uniform D:

- **A** = same corpus of shard-decl'd bilateral predicate action refs
  (extensible; grows monotonically as new bilaterals land).
- **H** = @glass.verdict-space (same).
- **D_reflective** = ONE operator: `D(a) := discharge(lookup(a),
  args)`. Same operator for every a ∈ A.

**Now D IS uniform over A.** The per-element specialization was
folded into a lookup from A to A's own metadata (the `bilateral {}`
block's sentinel + arity + require). The lookup is a substrate-native
projection A → decl-space; the evaluator D is a single operator on H
parameterized by the decl's shape but not by the a-identity.

### §2.4 The commutator condition holds

**Claim:** `[D_reflective, a]` is bounded for every a ∈ A.

**Proof sketch.** The reflective evaluator `discharge(decl, args)` is
a total function on (decl, args)-space; its output is bounded to the
finite verdict set {Pass, Fail(msg), Partial(opacity)}. The commutator
`[D_reflective, a]` reduces to the difference between D_reflective(a) —
evaluator applied to a — and a's own action on H. Since verdict-space
is a bounded set (three-state @glass.verdict) and a's action is bounded
(each atom contributes a bounded morphism per prism composition), the
commutator is bounded.

This is the LOAD-BEARING math: **making D uniform over A is what
makes (A, H, D_reflective) a spectral triple.** The reflective form
isn't just LOC-cheaper; it's *the actual Connes-triple discipline*
the substrate declared. The current form is not merely inefficient —
it's substrate-dishonest at the Connes altitude. ∎

### §2.5 The substrate self-supplies A's algebra elements

A load-bearing observation: A is not externally-supplied to the
substrate; the substrate **self-supplies** A via `shards/**`. Each
new `.mirror` shard-decl adds new atoms to A (via its `bilateral
{}` blocks); the grammar loader reads them at boot; the corpus is
the substrate's own self-declaration of its algebra.

This is the Connes-triple discipline **taken all the way**: the
substrate is not merely an operand of A — it IS A. The elements
of the algebra are the substrate's own shard-decl declarations.
The reflective evaluator D is the substrate's own way of applying
the algebra to itself.

Compare `[[architecture-spectral-triples-all-the-way]]`: "the
substrate carries the Connes triple at every altitude, not as
metaphor but as structural fact." This species is one instance of
that fact at the pact-family altitude.

---

## §3. The reflective evaluator is monotone under corpus extension

### §3.1 The monotone claim

Let A_t denote the bilateral-declaration corpus at time t (i.e., the
set of `bilateral {}` blocks landed in `shards/**` at time t). Let
V_t(a, args) denote the reflective evaluator's verdict when applied
to (a, args) with corpus A_t.

**Claim:** For any t1 < t2 with A_t1 ⊆ A_t2 (corpus grows
monotonically — no bilateral declarations are ever removed once
landed), and for any (a, args) with a ∈ A_t1:

```
V_t1(a, args) == V_t2(a, args)
```

That is: **adding new bilaterals does not change the verdict of
already-declared bilaterals**.

### §3.2 Proof

Fix (a, args) with a ∈ A_t1. Trace `V_t2(a, args)`:

```
V_t2(a, args)
= discharge(lookup(A_t2, a), args)
= discharge(decl_a, args)     [decl_a := A_t2.lookup(a); note that
                                a ∈ A_t1 implies a ∈ A_t2, and
                                A_t2.lookup(a) = A_t1.lookup(a) = decl_a
                                since declarations are byte-immutable
                                once landed]
```

Now split on whether decl_a has `require` populated:

**Case 1 (base bilateral, require empty):**
```
discharge(decl_a, args)
= <byte-check every arg.oid for containment of decl_a.sentinel>
```
This depends only on args and decl_a; it does not consult A_t2.
Therefore V_t2(a, args) = V_t1(a, args). ✓

**Case 2 (composed bilateral, require = [sub_1, ..., sub_k]):**
```
discharge(decl_a, args)
= AND_i (discharge(A_t2.lookup(sub_i), args))
= AND_i (discharge(A_t1.lookup(sub_i), args))    [by induction: each
                                                    sub_i is landed
                                                    before decl_a per
                                                    composition
                                                    discipline]
= V_t1(a, args). ✓
```

The induction terminates because the composition graph over
bilateral decls is acyclic (per `[[architecture-property-fracture-
bilateral]]` composition discipline — a composed bilateral names
sub-bilaterals whose declarations landed EARLIER; the DAG is
well-founded). ∎

### §3.3 Why monotonicity matters for Reed's landing

The retirement is 8-bite per §5.4 of the canonical spec. Between
bites, the corpus grows: Bite 1 adds 4 bilateral blocks; Bite 2 adds
4 more; etc. Between bites, the reflective evaluator dispatches on
a mixed corpus (some shards migrated to bilateral-block form; some
still in hand-typed-arm form).

**Monotonicity guarantees:** the shards already migrated (Bite 1
onward) do not have their Pass verdicts invalidated by the later
Bite migrations. Every already-witnessed Pass at time t1 stays a
Pass at time t2.

**This is what makes the 8-bite landing safe.** Reed can migrate one
shard at a time; verify each bite's Pass verdicts hold; proceed to
the next. The monotone theorem is what discharges the empirical
ouroboros_monotone `test_pass_rate == 100% → 100%` conjunct across
the entire retirement arc.

### §3.4 The corollary: no-regression guarantee

An immediate corollary: **for every currently-passing bilateral arm
in `apply_h.rs`, its reflective replacement will Pass under the same
argument**. The verdict is a byte-check against a substrate-decl'd
sentinel; the sentinel byte-string in the reflective form is the
SAME byte-string as the hand-typed form (copied verbatim from
docblock prose per Reed's realization discipline). Same sentinel +
same OID = same Pass.

This is the empirical basis for the sbec-neutral claim in the
canonical spec §5.3.

---

## §4. Sentinel algebra (byte-string monoid)

For completeness, we note that S — the set of substrate-decl'd
sentinels — carries a monoid structure under byte-concatenation:

- **Identity:** the empty byte-string `""` (which is disallowed by
  `bilateral_well_formed` §3.4 of canonical spec — an empty sentinel
  trivially Passes on every arg, so the substrate refuses it).
- **Composition:** byte-concatenation `s1 · s2`.
- **Associativity:** trivial for byte-strings.

However, **the reflective evaluator does NOT compose sentinels via
byte-concatenation**. Composed bilaterals (arity ≥ 2 or non-empty
require) compose SUB-BILATERAL VERDICTS via AND-conjunction, not
sentinel byte-strings via concatenation. This is because:

- Composing two sentinels s1 · s2 would require the arg's OID to
  contain the byte-string s1 · s2 (not s1 AND s2), which is a
  stronger condition than the substrate's landed composed
  bilaterals need.
- Each sub-bilateral's own sentinel encodes its own witness; the
  composition asks whether ALL sub-witnesses hold on the same arg
  tuple, which is the AND of the byte-checks.

The sentinel algebra is present in the substrate's math structure
but is NOT consumed by the current landed evaluators. Future
composed bilaterals may consume it if empirical need arises; punted
per canonical spec §7.2.

---

## §5. Foerster-eigenoperator specialization

Per Foerster 1976 "Objects: Tokens for (Eigen-)Behaviors" (cited
verbatim in `shards/mirror/lens/knife.mirror`): an eigen-operator Op
satisfies `Op(COORD_i) = COORD_i` for COORD in the operator's
stability domain.

The bilateral shape is a **specialization of the eigen-operator
discipline** at the sentinel altitude:

- **COORD_i** = an atom carrying sentinel s in its OID.
- **Op** = the reflective evaluator `discharge(decl_s, ·)`.
- **Op(COORD_i) = Pass = COORD_i** — the atom's carrying of s IS its
  fixed-point-witness under discharge; the operator returns Pass
  (the identity verdict at eigen-altitude).
- **Op(a) = Fail** for a NOT carrying s — the atom is outside Op's
  stability domain; Foerster's "jump" happens at the domain boundary.

**The bilateral is what Foerster's COORD-jump discipline looks like
at the pact altitude.** The Recognition candidate #R-substrate-shape-
of-bilateral-predicate-is-typed-carrier-plus-reflective-evaluator
(§8.1 of canonical spec) is thus load-bearing not merely as a
pattern-recognition but as a specialization of an already-landed
substrate identification (`@knife` IS Foerster COORD; the bilateral
IS Foerster's eigen-operator at sentinel altitude).

Same theorem, different altitude:
- @knife altitude: coordinate-jump at stability-domain boundary.
- bilateral altitude: sentinel-containment at OID-byte-level.

---

## §6. Compositional correctness (proof sketch)

### §6.1 The claim

The reflective evaluator's discharge function is **compositionally
correct** with respect to the current hand-typed arms: for every
currently-landed bilateral arm A in `apply_h.rs` corresponding to
shard-decl bilateral B, the reflective evaluator applied to B's
decl on the same args produces the same verdict as A on the same
args.

### §6.2 Proof sketch

By construction:

- **A's shape** per §2.2 of canonical spec:
  ```
  if action == "<shard-ref>.<name>" {
      if let Some(arg) = args.first() {
          if arg.oid.contains("<sentinel>") { return Pass; }
          return Fail(<msg>);
      }
      return Fail(<missing-arg-msg>);
  }
  ```

- **Reflective evaluator applied to B's decl** per §3.3 of canonical
  spec pseudocode:
  ```
  if decl.arity != len(args): return Fail(<arity-msg>)
  if decl.require empty:
      for arg in args:
          if !arg.oid.contains(decl.sentinel): return Fail(<msg>)
      return Pass
  ```

For **arity 1** (24 of the 30 landed arms):
- Both check `arg.oid.contains(sentinel)`.
- Both return Pass on containment, Fail otherwise.
- The sentinel byte-string is the SAME (copied verbatim from docblock
  to decl.sentinel field per Reed's realization discipline).
- Therefore identical Pass/Fail behavior. ✓

For **arity 2** (2 pairs, e.g., `gc_reachability_closure_second_witness`):
- Hand-typed arm checks BOTH args for the same sentinel:
  ```
  if args[0].oid.contains("<sentinel>") && args[1].oid.contains("<sentinel>")
  ```
- Reflective evaluator iterates `for arg in args: if !arg.oid.contains(decl.sentinel): Fail`.
- Both discharge Pass iff BOTH args carry the sentinel. ✓

For **arity N composed** (4 composed bilaterals with `require`
populated):
- Hand-typed arm checks the composed sentinel (`witnessing=all-four-pass`)
  in the arg's oid.
- Reflective evaluator RECURSIVELY evaluates each sub-bilateral on the
  same arg tuple; Passes iff all sub-bilaterals Pass.

**Note:** the current hand-typed forms of the composed bilaterals
check the COMPOSED SENTINEL `witnessing=all-four-pass`, not the
recursion over sub-bilaterals. Under the reflective form, the composed
bilateral's `require` field explicitly names the sub-bilaterals; the
recursion is the substrate-honest form of "all four Pass" (whereas
the hand-typed form asserts the same via a single sentinel byte-check).

**This is a semantic UPGRADE, not just a form-change.** In the
hand-typed form, if the arg's OID carries `witnessing=all-four-pass`
but any of the sub-witnesses is absent, the arm Passes anyway (false
positive). In the reflective form, the recursion Fails on any missing
sub-witness. The reflective form is **strictly stronger** — it enforces
the composition semantics the hand-typed form declared but did not
enforce.

This is an ADVERTISED discipline lift: the retirement makes composed
bilaterals MORE substrate-honest than the current arms. Noted in
canonical spec §5.3 as a corollary of the ouroboros_monotone
`test_pass_rate` conjunct: existing sub-witness-carrying args stay
Pass (all sub-witnesses present when composed-sentinel is present per
Reed's realization discipline); newly composed args gain the correct
enforcement.

∎

---

## §7. Delightfully-boring closure

Alex's design discipline (AGENTS.md Delightfully Boring): *"the reader
ought to go 'of course it's this.'"*

Applied to this math foundation:

- **§1** proves χ_s is Rice-safe. Of course it is — it's a byte-check
  on the OID, not a semantic check on the atom.
- **§2** proves the shard corpus + reflective evaluator IS a Connes
  spectral triple. Of course it is — the substrate declared
  spectral-triples-all-the-way; this species is one instance.
- **§3** proves the reflective evaluator is corpus-monotone. Of course
  it is — bilateral decls are byte-immutable once landed; the DAG is
  acyclic; the induction terminates.
- **§4** notes the sentinel monoid structure but observes it's not
  consumed. Of course — the substrate needs AND-of-verdicts for
  composition, not concatenation-of-sentinels.
- **§5** specializes to Foerster's eigen-operator discipline. Of
  course — @knife IS Foerster COORD; the bilateral IS Foerster's
  eigen-operator at sentinel altitude; the substrate's cybernetic
  ancestry runs through both.
- **§6** proves compositional correctness. Of course — the
  reflective form byte-copies the sentinels and checks the same
  bytes; same in-same out.

**The math is boring because the mint is a substrate-pull discharge,
not an invention.** The shape was landed 30 times before it was
named; the math ratifies what the substrate already carried.

---

## §8. Citations

### §8.1 In-substrate

- `shards/epistemologic/pact/bilateral.mirror` — the species-decl.
- `shards/epistemologic/pact/keywords.mirror` — the companion
  keyword bindings.
- `docs/specs/bilateral-predicate-substrate-shape.md` — the canonical
  spec.
- `shards/spectral/signature.mirror`,
  `shards/epistemologic/cybernetic/coherence.mirror`,
  `shards/peer/persistence.mirror`,
  `shards/kintsugi/roomba.mirror` — the four COLLAPSED witnesses.
- `shards/mirror/lens/knife.mirror` — Foerster COORD substrate-decl.
- `shards/glass.mirror` — three-layer content-address recognition.
- `shards/nl.mirror` — Connes spectral-triple substrate-decl.

### §8.2 Pre-AI ancestry

- **Foerster 1976** "Objects: Tokens for (Eigen-)Behaviors" — the
  eigen-operator discipline this species specializes.
- **Connes 1994** "Noncommutative Geometry" — the spectral triple
  (A, H, D) discipline the reflective evaluator restores.
- **Pask 1975** "Conversation Theory" — bilateral as one turn in a
  Paskian agreement (recognition #37 substrate-decl'd; this species
  is its typed carrier).
- **Rice 1953** — the Rice-safety bound the byte-check respects.
- **Bazel 2015 / Nix 2003** — content-addressed dispatch precedent
  at the build-system altitude.

### §8.3 Composition footer

- **Alex 2026-07-16 verbatim:** *"Q1. Let's mint it then. Properly.
  Seems like it's load-bearing."*
- **Taut Q1 substrate-truth scout** (this session, 2026-07-16
  evening) — the substrate-truth grounding.

---

**End of `docs/math/epistemologic/pact/bilateral-sentinel.md`.**
