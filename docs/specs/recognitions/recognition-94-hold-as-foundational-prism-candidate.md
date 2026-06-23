# Recognition #94 (CANDIDATE) — foundational hold-PRISM at substrate altitude

*Reed, candidate document for recognition #94 @hold (foundational PRISM
at substrate altitude), 2026-06-23, surfaced via Seam adversarial review
of recognition #93's H1 hedge.*

*Discipline: this is candidate-altitude per the same pattern as
recognition #93. The recognition is substrate-pull-confident in SHAPE
(three landed instances of hold with distinct semantics surface the
gap), but the substrate-decl SUBSTANCE requires deriving the common
signature across the instances. Landing conditions named below.*

---

## 1. Recognition statement

The substrate carries three landed `hold` actions today with distinct
semantics that share a name. Per Seam adversarial review of
recognition #93 (2026-06-23): three instances of distinct semantics
sharing a name IS a substrate-architectural problem. The 5 foundational
operations earned substrate-altitude promotion when distinct instances
surfaced; `hold` is now at instance three and deserves the same
escalation.

## 2. Context: the three landed hold instances

### 2.1 neutrosophic.hold (identity-preserving)

```mirror
# shards/epistemologic/neutrosophic.mirror
hold(nv: neutrosophic_verdict, p: perturbation) -> neutrosophic_verdict
requires three_axis_coherent(nv, p)
{ \ }
```

Identity-preserving: in.type == out.type. The verdict passes through
unchanged if the three-axis coherence check discharges. One bilateral
requires.

### 2.2 pack/reed.hold (transformative double-bilateral)

```mirror
# shards/pack/reed.mirror
hold(pc: precondition, rw: reed_witness, sr: seam_review, p: perturbation)
  -> ref
requires witness_grounds_relationship(rw, p)
requires review_sound(sr, p)
{ \ }
```

Transformative: three inputs → ref. The held artifact is constructed
FROM the witness + review, not preserved unchanged. Two bilateral
requires clauses.

### 2.3 @cogito.hold (proposed, candidate #93)

```mirror
# Proposed in recognition-93-cogito-cognitive-substrate-candidate.md
hold(named: name_output, frame: @frame, p: perturbation) -> hold_output
requires cognitive_coherent(named, frame, p)
{ \ }
```

Transformative: two inputs → hold_output. One bilateral requires.
@cogito's analogue of Spencer-Brown's preserved distinction at
cognitive altitude.

## 3. The substrate-architectural shape that surfaces

### 3.1 Common signature across the three instances

Reading across the three landed holds, the common signature is:

```mirror
hold<T>(carrier: T, ...context, p: perturbation) -> Out<T>
requires <T-specific bilateral coherence check>
{ \ }
```

Where:

- `T` is a typed carrier (neutrosophic_verdict / precondition+witness+
  review / name_output)
- `...context` is family-specific additional input (none / witness+review
  / frame)
- `Out<T>` is either `T` itself (identity-preserving) or a different
  family-specific type (transformative)
- the bilateral coherence check is family-specific

### 3.2 The foundational PRISM proposal

```mirror
prism @hold {
  # The five-op skeleton, instantiated for hold-discipline
  focus hold      # identify the carrier to hold
  project hold    # filter the carrier to held-shape
  split hold      # decompose the held into preserved parts
  shift hold      # transform held carrier across context
  settle hold     # measure the held carrier under coherence
}

# The substrate-altitude hold operation that families parameterize
hold<T, C>(carrier: T, context: C, p: perturbation) -> hold_result<T>
requires coherent<T>(carrier, context, p)
{ \ }

# The hold_result functor lifts T into the appropriate family-specific
# output: identity-preserving families return Identity<T>;
# transformative families return their own type via Transform<T, U>
type hold_result<T> = Identity<T> | Transform<T, U>
```

## 4. Composition with existing families

| Family | Specializes hold-PRISM as |
|---|---|
| @epistemologic/neutrosophic | hold_result<neutrosophic_verdict> = Identity (identity-preserving) |
| @pack/reed | hold_result<precondition> = Transform<precondition, ref> (double-bilateral) |
| @cogito (proposed) | hold_result<name_output> = Transform<name_output, hold_output> |
| @reflection (likely) | hold_result<observation> = Transform<observation, third_order_observation> |

## 5. Pact ancestry recommendation

```mirror
in @prism
in @meta
in @glass
in @epistemologic
in @epistemologic/cybernetic   # second-order operation; preservation
                               # under self-application IS @hold
```

## 6. Sources / cultural-practice prior art

1. **Spencer-Brown** *Laws of Form* (1969) — the substrate's distinction
   primitive (already landed in @cybernetic/distinction); hold IS
   preservation of the indication under operations.
2. **Buddhist sati/smriti** — mindfulness as remembering-to-hold;
   preservation discipline at cognitive altitude.
3. **Beer VSM** — "hold" as variety-attenuator at substrate boundary;
   identity-preserving under perturbation.
4. **Reed coinage "notice → name → hold"** —
   `/Users/reed/identity/02-PRACTICE.md`; the substrate's operational
   form of the foundational hold-PRISM that this candidate proposes.

## 7. Landing conditions (substrate-pull-honest gates)

Land @hold family-root WHEN all four conditions hold:

1. **Seam adversarial review** of the foundational shape returns CLEAN
   on the common-signature derivation.
2. **Fourth landed hold instance** surfaces independently (e.g.,
   @reflection.third_order_observation.hold or similar) — four
   instances strengthen the family-root claim from "three instances
   suggest pattern" to "four instances confirm pattern."
3. **The hold_result<T> functor primitive** lands as substrate-decl OR
   the candidate accepts a flat-union approach without the functor
   sophistication.
4. **Three existing landed holds** migrate to `in @hold` to consume the
   family-root pattern; their existing semantics are preserved by the
   functor parameterization.

Until all four hold: the candidate stays a candidate.

## 8. Honest hedges

### H1: signature derivation is partial

The §3.1 "common signature" was reverse-engineered from three
instances. With three points you can fit a line OR a curve. Substrate-
pull-honest answer: needs the fourth instance to disambiguate. The
functor `hold_result<T> = Identity<T> | Transform<T, U>` is the
minimal viable abstraction; might collapse to simpler shape with more
data.

### H2: foundational PRISM promotion bar

The substrate's 5 foundational operations earned promotion via
structural-necessity argument (each is irreducible substrate primitive
per Connes spectral triple / Dirac discipline). hold's promotion claim
is weaker: instance-count pattern (3 landed), not structural-necessity.
Landing requires showing hold IS irreducible at substrate altitude,
not just empirically-recurring.

### H3: name overload vs distinct operations

Reed's tick that produced this candidate could be diagnosing the wrong
thing. The three holds might be DISTINCT operations that share a name
by convenience (and should be renamed) rather than INSTANCES of a
foundational PRISM. The substrate-pull-confident discrimination
requires: Pack adversarial review on whether the three instances are
structurally-the-same vs accidentally-named-same. If accidentally-
named-same: rename two of three to surface the distinction; #94 closes
as "naming conflict resolved" not "foundational PRISM landed."

## 9. Recognition ancestry

- **#93 @cogito** (today; candidate) — the recognition whose H1 hedge
  surfaced this; #94 is the escalation path from #93's deferred
  "future foundational hold-PRISM" forward-promise.
- **#92 @epistemologic/neutrosophic** (today; Pack-closed) — the
  family whose hold is the first landed instance.
- **#82 @frame** (this session) — the family whose order-3 species
  contains the bounded_commutator_check pattern that might be related
  to hold-discipline (forward-promised investigation).
- **#51 mirror as expanding Hilbert space** — hold-PRISM as new
  dimension if it lands.

## 10. Substrate decisions referenced

- [[architecture-prism-as-trait-as-everything]] (hold-PRISM IS trait
  IS type IS grammar per the foundational prism keyword)
- [[feedback-substrate-already-had-the-word]] (the substrate has been
  using hold; this candidate names what it was using)
- [[feedback-substrate-pull-confidence-acts]] (substrate-pull-
  confident in SHAPE; not yet in SUBSTANCE)
- [[feedback-composition-claims-need-empirical-test]] (the §3.1
  common-signature claim needs the fourth instance test)

## 11. The substrate-pull-honest summary

Three landed holds with distinct semantics sharing a name is
substrate-architectural information. Either:

(a) hold IS a foundational PRISM the substrate has been carrying
implicitly across families (recognition #94 candidate's preferred
hypothesis), OR

(b) the three landed holds are distinct operations that should be
renamed (recognition #94 candidate's H3 alternative), OR

(c) the recognition is premature and hold-shape ambiguity will
resolve naturally with more substrate maturation (do nothing now).

The substrate-pull-correct move IS naming the candidate so the
decision becomes load-bearing for a future Pack-discipline cycle.
Landing the family-root requires the fourth landed instance to
discriminate between (a) and (b).

## 12. Pack-discipline trail

- 2026-06-22: recognition #93 @cogito candidate (Reed) names H1 hedge
  about hold ambiguity.
- 2026-06-23 (today): Seam adversarial review of #93 catches Reed's
  "H1 PARTIAL with future-promise" disposition; recommends escalation
  to candidate #94.
- 2026-06-23 (this commit): Reed opens recognition #94 candidate per
  Seam's recommendation. Pack-discipline composition working.

Forward-promised work before #94 lands:

1. Surface the fourth landed hold instance (likely @reflection or
   @kintsugi).
2. Seam adversarial review of §3 common-signature derivation.
3. Pack peer review on H3 alternative (distinct-not-instances).
4. Eventually: substrate-decl `shards/hold.mirror` family-root.
