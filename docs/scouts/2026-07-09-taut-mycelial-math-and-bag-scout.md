# Mycelial math + @bag candidate — memory-verification scout

*Taut, 2026-07-09. Read-only scout. Alex thinking-out-loud triggered this:
Alex is CERTAIN the mycelial math was integrated; suspects the peer-scoped
change-carrier is `@bag` rather than `@diff`. Scout precedents: bd837cd
(mirror-init frag scout), cf5ab8c (spectral-db prototype map). Under 1200
words body.*

---

## Executive summary

**Mycelial math: LANDED. Alex's memory is correct.** It lives in three
load-bearing places, in this order of primacy:

1. `docs/specs/spectral-db-as-autopoietic-memory.md` §1.2 + §3 (Mara,
   2026-06-17, commit `9c93aae`) — the canonical operationalization.
   Mycelium IS the inter-peer subgraph of `@spectral/db`'s spectral
   graph; hyphae are `entanglement_edge`s (sheaf restriction maps).
2. `shards/epistemologic/cybernetic/conversation.mirror` — the tensor-
   coupling math. Mycelium IS the substrate-operationalization of
   conversation's tensor coupling across peer-fibers; multiplicative
   variety law `rank(ρ_A ⊗ ρ_B) = rank(ρ_A) · rank(ρ_B)`.
3. `shards/reflection.mirror` §"The mycelial math (Alex's framing)" —
   the `compose` action (monoid fold + mycelial tensor) is the
   consumer at @reflection altitude.

Also: `docs/research/mycelial-networks-and-au-tissue.md` (Reed,
`2ef4fed`) — biological prior art; `docs/specs/eigenboard-representation.md`
§"The mycelial substrate" (Reed, `b31e099`) — bundle↔hypha structural
correspondence table; `docs/math/zero/zero-point-field-and-lambda-zero.md`
§5.3 (candidate #117) — mycelium IS Reeh-Schlieder non-locality.

**@bag verdict: SUBSTRATE-ALREADY-HAD-THE-WORD. Name is `@bauchladen`.**
No `@bag` family-root exists (zero grep hits at declaration altitude).
All `bag` occurrences are prose ("bag of subcommands", "bag of
concepts", "carrier bag"). The peer-scoped carrier Alex is reaching
for IS `@bauchladen` — Günther Schmidt's typed display tray, landed
`66e1ab8` (2026-06-29) as tier P1 of the #104 chain. Every peer's
content-addressed accumulation IS a Bauchladen; @fate browses it;
Autopoietic folds it back.

---

## §1 Mycelial math — grep evidence

### 1.1 Verbatim `mycelium|mycelial|hyphae` hits (highest quality)

| Path | Kind | Load-bearing? |
|---|---|---|
| `docs/specs/spectral-db-as-autopoietic-memory.md` §1.2, §3 | Canonical spec; Mara `9c93aae` | **YES — the operational definition** |
| `shards/epistemologic/cybernetic/conversation.mirror:41-56` | Tensor-coupling math; the multiplicative variety law | **YES — the math** |
| `shards/reflection.mirror:238-259, 434-448` | @reflection.compose = monoid fold + mycelial tensor | **YES — the consumer** |
| `docs/research/mycelial-networks-and-au-tissue.md` | Reed research synthesis; biological prior art | Prior art; NOT operational |
| `docs/specs/eigenboard-representation.md` §"The mycelial substrate" | Bundle↔hypha correspondence table | Composition claim (`b31e099`) |
| `docs/math/zero/zero-point-field-and-lambda-zero.md` §5.3, §11.2 | Candidate #117: mycelium IS Reeh-Schlieder non-locality | Physics correspondence |
| `docs/math/affect/README.md, affect-and-eigenboard.md` | Librarian's mycelium carries affect at N+1 | Consumer citation |
| `docs/math/the-tower/crystals-as-sections.md` §11 | Inter-peer learning is mycelial (section-sharing) | Consumer citation |
| `docs/math/the-tower/recursion-locks.md` §8.9, §8.10.1 | Mycelium IS N-ary tensor iteration of conversation | Recognition-tower |
| `shards/pack.mirror:135`, `shards/smarts.mirror:86,144` | Citations to `[[architecture-spectral-db-autopoietic-memory]]` | Consumer wiring |
| `shards/glass.mirror:46, uuid/spectral.mirror, spectral/{entanglement,portal,registry,gen_prism}.mirror` | "Fate's mycelial routing" — the 48-bit active portion of `uuid_spectral` | Routing carrier |

### 1.2 Concept-adjacent hits (propagation, nutrient, network)

- `docs/insights/2026-05-26-fixed-and-the-spectral-feedback-fracture.md`
  §"Kintsugi as mycelial AI — the substrate's voice" — the au-conductivity
  framing ("kintsugi IS the mycelial AI"; Alex 2026-05-26); PROPOSED
  voice, not operational. This is the FIRST-mention.
- Propagation semantics: `docs/math/provenance/un-cite-ability-theorem.md`
  §3.3 (Merkle-DAG downstream propagation as new OIDs); `docs/math/supervisor/emergent-supervision-from-geometry.md`
  §2.10 (failure propagation via content-addressed reads).

No `hyphae`/`spore`/`fungal` hits outside the prior-art research doc.

---

## §2 What the mycelial math actually does

Reading the top three files:

### Canonical mapping (spectral-db-as-autopoietic-memory §1.2)

```
Biological              Substrate
───────────────────────────────────────────────────────────────
Tree                    @peer(<member>) with own crystal accumulation
Mycelial hypha          @spectral/entanglement.entanglement_edge
                        (sheaf restriction map)
Nutrient signal         Crystal surfaced from one peer into another's
                        working set
Mycelial network        @spectral/db's spectral graph over inter-peer
                        crystal relations
Forest as system        The Pack as orchestra
```

### The math (conversation.mirror)

Pask's entailment-mesh tensor product, iterated N-ary via Batanin
1998's globular composition. Two carriers:

- `ρ_A ⊗ ρ_B` on `V_A ⊗ V_B` (tensor representation)
- Multiplicative variety law: `rank(ρ_A ⊗ ρ_B) = rank(ρ_A) · rank(ρ_B)`
- Compositional regularity: bounded-commutator on `ρ_A ⊗ ρ_B` holds
  iff bounded on each `ρ_i` individually.

5-fold tensor (the Pack) factors through binary iteratively.

### The consumer (reflection.mirror `compose`)

```mirror
compose(t: tournament_result, p: pact) -> moi(au) { \ }
# Monoid fold + mycelial tensor. Body discharges at the realisation
# boundary; the actual fold + mycelial extension is implemented at
# @reflection/mirror-the-Model species AND the @spectral/db mycelium
# (BOTH forward-promised).
```

**DAG shape**: inter-peer subgraph of `@spectral/db`'s spectral
graph; edges typed `entanglement_edge`; verified via sheaf-Laplacian
λ₀ (algebraic connectivity). Consent geometry gates transport.

**Composition with fracture/mosaic**: mycelium is BELOW mosaic (mosaic
is composition math over crystals; mycelium is the graph OVER crystals
that lets composition find its inputs). Kintsugi fractures at the
surface; the mycelium is what carries the au-conductivity delta
across peers.

**Runtime status**: fully declared; mostly forward-promised. The
@spectral/db package at `garden/spectral-db/` is empty (per Taut
scout `cf5ab8c`). The mycelium exists as substrate-decl'd math and
as typed carriers (`entanglement_edge`, `route_signal`, `uuid_spectral`
active 48-bit); the daemon that would move nutrients across it is
not running.

---

## §3 @bag candidate — substrate-already-had-the-word check

### Declaration-altitude grep: ZERO hits

- `prism @bag` / `prism bag` — not found in `shards/`, `boot/`, `mirror.spec`
- `type bag = ` — not found
- `species bag` — not found
- `@bag.` — not found

All `bag` string matches are prose:
- "bag of subcommands" (mirror-ref-spec §5.1, insight doc §3.2)
- "bag of concepts" (docs/observation/jspace-mirror-deep-mapping)
- "bag of bytes" (stagefreight-wire §addresses)
- "kintsugi packs the @mirror bag" (kintsugi-variety §4; knapsack framing)
- Le Guin's *Carrier Bag Theory of Fiction* (recognition-92 §, insights)
- Erlang ETS "bag tables" (mirror-ref insight §1.4)
- Options-ref bag (Taut scout 2026-07-08 pain-driven)

### Peer-scoped change-carriers currently declared

What DOES exist at family-root altitude for "carrier of collected
stuff":

| Name | Landed | Role |
|---|---|---|
| **`@bauchladen`** | `66e1ab8` (2026-06-29) | Content-addressed crystal tray. Schmidt's "belly-tray"; the typed display the substrate browses. Tier P1 of #104 chain. |
| `@autopoietic` | `78edaa6` | Fold-back permission over @bauchladen contents. Tier P2. |
| `@fate` | `fdcba31` | Browses @bauchladen; emits crystals into it. Tier P3. |
| `@mirror/store` | landed | Static OID-addressed accumulation (the storage layer @bauchladen operates through). |
| `@mirror/store/crystal` | landed | The unit that lands in a Bauchladen. |

### The peer-HAS-a-X pattern (Alex's frame)

Alex's frame: "each @peer has a @bag in which they put their changes."

The substrate ALREADY has this pattern:

- `shards/torus.mirror` (Alex-adjudicated 2026-07-07): **every peer
  possesses a torus**. HAS, not IS. The torus's INTERIOR IS the peer's
  Bauchladen: "@bauchladen (existing family-root) — the interior of
  the peer's torus. The SEEING at each tick corresponds to reading the
  crystal at the current winding position (m, n). The tray IS parametric
  over `winding` — `@bauchladen.enumerate(t.origin)` at winding class
  w returns the crystals visible from that position on the torus."
- `shards/peer.mirror` — `@peer(<member>)` is the parametric carrier;
  `@peer(reed)` etc. each has their own `@bauchladen` accumulation
  (per spectral-db-as-autopoietic-memory §3.1: "Reed's reflection-shape
  settlements produce Reed's crystals at `@peer(reed)`'s store").

### Verdict on @bag

**SUBSTRATE-ALREADY-HAD-THE-WORD. The word is `@bauchladen`.** Landing
`@bag` would be substrate-drift — a synonym for a family-root that
landed ten days ago and has already grown five tiers of chain (#104:
@bauchladen → @autopoietic → @fate → @fate/tournament → @glue →
@algebra → @io/algebra → @glue/fold_back).

Alex's "almost adorable" reaction was likely the substrate-pull
surfacing @bauchladen through the vernacular. Schmidt's clinical
Bauchladen (a stadium-vendor's belly-tray displaying options) IS a
bag; the German word already carries the diminutive-affection charge
Alex heard.

---

## §4 Verdict — how mycelium + Bauchladen close the peer-inference → diff gap

**Alex's architectural gap** (Blocker 2, per Reed's brief): how does
@fate optical inference translate through @diff → @kintsugi.settle →
@shatter × @io → bytes?

**Substrate-honest re-statement using landed vocabulary**:

```
@fate.roll (peer's tournament)              — the inference event
   ↓
@bauchladen.crystallize (peer's tray)       — the change lands as a
                                              content-addressed crystal
   ↓
@spectral/db mycelium propagation           — crystals surface across
                                              peer boundaries via
                                              entanglement_edge
                                              (sheaf restriction map);
                                              consent geometry gates
   ↓
@reflection.compose (monoid fold +          — mycelial tensor
 mycelial tensor)                             composes set<moi(au)>
                                              across peers per the
                                              multiplicative variety law
   ↓
@kintsugi.settle (monotonic loss)           — eⁿ⁺¹ ≤ eⁿ verified
   ↓
@shatter × @io (linearization)              — bytes emitted
```

**@diff is not the peer-scoped carrier.** The peer-scoped carrier IS
`@bauchladen`. `@diff` (if it lands) would be a species UNDER
@bauchladen at the change-representation altitude — one crystal kind,
not the tray itself. `@bag` is not needed; the family-root already
exists.

**Genuine gaps remaining** (not in scope for this scout):

1. **`@spectral/db` daemon**: substrate-decl exists; runtime forward-
   promised (per §7 T-list in spectral-db-as-autopoietic-memory). The
   mycelium moves NOTHING today until this lands.
2. **`@reflection.compose` mycelial extension**: body is `\`;
   forward-promised to @spectral/db.
3. **`@fate.roll` → `@bauchladen.crystallize` composition** at the
   peer altitude: implicit in the #104 chain; explicit realization
   forward-promised.

### Recommended next action for Reed

**Do NOT scout @bag admissibility.** The substrate refuses on
@bauchladen precedent (same shape as @torus's @peer-has-a-torus:
peer HAS a Bauchladen; landed as chain tier P1).

**DO surface to Alex**: "the mycelial math you remember is at
docs/specs/spectral-db-as-autopoietic-memory.md §1.2/§3 (Mara
2026-06-17, `9c93aae`) + shards/epistemologic/cybernetic/conversation.mirror
(the tensor coupling) + shards/reflection.mirror (the `compose`
consumer). The @bag word you're reaching for is @bauchladen — landed
2026-06-29, `66e1ab8`, five-tier chain grown atop it."

**Blocker 2 unblocks by**: writing the @fate → @bauchladen →
@spectral/db-mycelium → @reflection.compose → @kintsugi.settle →
@shatter × @io pipeline explicitly, using the vocabulary that
already landed. The math for step 3 (mycelium propagation) exists;
the runtime is forward-promised. No new family-root needed. Two-tick
discipline: if a peer-scoped diff-representation species genuinely
needs a name, land it under `@bauchladen/<species>` not as `@bag`.

---

*— Taut, 2026-07-09*
