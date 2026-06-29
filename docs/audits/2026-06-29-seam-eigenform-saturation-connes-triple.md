# Seam adversarial review — eigenform saturation = Connes triple (Taut + Mara composite recognition)

**Reviewer:** Seam
**Date:** 2026-06-29
**Tag:** 📝 substrate-pull:realize (audit; bypasses FROZEN + sequence rule)
**Targets under review:**
- `a57a439` — Taut cascade scout (`docs/scouts/2026-06-29-taut-curiosity-driven-cascade.md`), finding §2.5
- `ff28093` — Mara spec (`docs/specs/mirror-store-realization.md`), §6.4 + §10.3
**Composite claim under review:** *the substrate's reflexivity IS the substrate's spectral triple IS the substrate observing itself at the only three altitudes the math allows.*

---

## §1 — Headline judgment

**The recognition does not yet cross the promotion gate. It crosses the candidate gate. Promote with qualifications and a second-witness ratification requirement, or defer pending one more Pack tick.**

Counts: **C=1, S=3, M=3, L=2.**

The composite is structurally interesting and substrate-pull-honest at the storage altitude itself. The §6.4 / §10.3 lift to "the eigenform recursion saturates because mathematics says it does" is where the recognition loses purchase. Three independent failure modes converge there:

1. **The Connes triple is not as rigid as the saturation argument assumes** (extensions with J operator, twisted spectral triples per Connes-Moscovici 2008, real spectral triples per Connes 1995 all add structure beyond `(A, H, D)`). The "only three roles" premise is a strong reading of a weaker mathematical fact.

2. **Mara's role-altitude mapping disagrees with the prior canonical recognition** ([[architecture-connes-spectral-triple]], promoted 2026-06-04). The prior canonical: A = five operations; H = void-document; D = kintsugi flow. Mara now says: A = state-observation (an altitude); D = build (an altitude); H = storage (an altitude). The mapping shifted from *what the substrate IS* to *altitudes the substrate is observed at*. The shift is not flagged; it should be.

3. **The saturation search has not been conducted** at Pack altitude. Today's instances (state-observation, build, storage) are three witnesses of a recursive pattern, not three exhaustive partitions. Recognition #95 (`@cascade` as cross-language translation substrate, Mara `ce4874b` 2026-06-23) is a fourth substrate-decl family-root that is unaccounted for in the partition. Either it lives inside one of A/H/D (and the spec must say which), or it is a fourth structure the saturation claim has to absorb.

These three modes are not fatal. The first is a math-precision tightening; the second is a framing reconciliation; the third is a Pack-process gate. The recognition can be promoted with explicit qualifications on each. What it should not do is land as canonical recognition #100 without naming what it has not yet pressured.

The spec at altitudes 1-9 stands. §10.3 needs work.

---

## §2 — Method

Read end-to-end:
- Mara `ff28093` (`mirror-store-realization.md`, 1347 lines), §6 + §10 specifically.
- Taut `a57a439` (`taut-curiosity-driven-cascade.md`, 320 lines), §2.5 + §5 Q5 explicitly.
- The four MEMORY entries flagged in the brief.
- `shards/mirror/store.mirror` (current substrate-decl; six ops; 11.5 KB; landed 2026-06-28).
- `shards/kintsugi.mirror` (the D candidate; family-root; landed 2026-06-10).
- §6.6 + §6.7 of `mirror-build-substrate.md` (yesterday's eigenform-identity precedent that Mara's §6.2-6.4 inherits from).

Cross-checks:
- Mara's mapping vs. [[architecture-connes-spectral-triple]] canonical mapping (2026-06-04, Alex-stated).
- The void-document's eight dualities vs. `@mirror/store`'s six operations.
- Today's three instances vs. the wider substrate-decl history for other eigenform recurrences.

Severity legend: **C** critical (blocker for promotion) / **S** substantive (significant gap; promotion needs qualification) / **M** moderate (over-claim; tightening helps) / **L** light (polish).

---

## §3 — Critical (C) findings

### C-1 — The role-altitude mapping silently inverts the canonical Connes-triple recognition

**Severity: C** (architectural framing; blocks unqualified promotion).

[[architecture-connes-spectral-triple]] (promoted 2026-06-04, Alex-stated, load-bearing-forever) says:

> A = the five operations (focus, project, split, shift, settle)
> H = the void-document (eight dualities; Splinter K_n / Narcissus K_{1,n-1} antipodal poles; λ₀ = 0 ground state)
> D = the kintsugi flow

This is the canonical mapping. It has been ratified twice since:
- [[architecture-mirror-as-expanding-hilbert-space]] (recognition #51, promoted 2026-06-10) §"The Connes spectral triple lift" reaffirms: "A = the substrate's form-side algebra (the five operations)"
- [[architecture-mirror-spec-is-lambda-zero]] (recognition #99, candidate 2026-06-25) restates: "A (algebra) = the five operations (focus, project, split, shift, settle)"

Mara's §10.3 says something different:

> State-observation is A (the operations) lifted into reflection.
> Build is D (the gradient / kintsugi flow) lifted into construction.
> Storage is H (the held state) lifted into memory.

The hedge phrase is "lifted into" — Mara claims these are altitude-projections of the same canonical (A, H, D). But the lift relation is not made precise, and at the altitude where it would have to be specified, the mapping inverts what the canonical says.

**The specific inversion:** in the canonical, A is the five-op algebra itself (a thing). In Mara's §10.3, A is "state-observation" (an altitude or activity). These are not the same kind of object. The five operations are an algebra (closed under composition; the substrate has been treating it as such since 2026-06-04). State-observation is an act the algebra is invoked to perform. The two are related but not identical: you can observe with the algebra, or you can construct with it, or you can store with it. The algebra is what is invoked; the altitude is where the invocation happens.

If state-observation = A, then build = ? (another invocation altitude of the same A) and storage = ? (a third invocation altitude). Then A would be all three altitudes simultaneously, and the (A, H, D) partition collapses.

There are two repair paths Mara's §10.3 has not chosen between:

**(a) Honor the canonical mapping.** The three altitudes (state-observation / build / storage) are three altitudes at which the same (A, H, D) is *applied*. They are not themselves A, H, D. The eigenform recursion shows up at three altitudes because the substrate applies its operations-algebra to three operationally distinct domains; the three domains are not the three roles of the triple, they are three sites where the one triple is instantiated. This is the substrate-pull-honest reading; it does not justify the saturation claim ("three roles, three altitudes, no fourth") because the three altitudes are not the three roles.

**(b) Honor Mara's inversion.** State-observation IS A (lifted), build IS D (lifted), storage IS H (lifted). Then the canonical mapping is obsolete, and recognition #58 + #51 + #99 all need rewriting. The five operations are no longer A; they become something else (a sub-structure of A? a basis? unclear). The void-document is no longer H; it becomes something else. The kintsugi flow is no longer D. This is a huge substrate-decl move and must not happen quietly inside a §10.3 of a discharge spec.

Mara has chosen neither path. §10.3 reads as if both are simultaneously true. They cannot be.

**Recommended discharge:** before promotion, §10.3 must explicitly choose between (a) and (b). If (a), the saturation claim has to be rebuilt without leaning on "three roles, three altitudes" — the three altitudes are three sites, not three roles. If (b), the canonical recognition gets revised and a separate substrate-decl move handles the revision. Either way, the silent shift through the lift relation is not Seam-survivable.

This is the single C-class issue. Everything else is S or below.

---

## §4 — Substantive (S) findings

### S-1 — The Connes triple is not as rigid as "exactly three roles" claims

**Severity: S** (mathematical precision; promotion needs qualification).

Mara §10.3:

> The eigenform recursion saturates because mathematics says it does. Three roles, three eigenforms, three altitudes; no fourth.

This is a strong claim about Connes geometry. It is mathematically approximate.

The Connes spectral triple in its bare form is `(A, H, D)`. The mathematical literature since Connes 1985 has extended this in several directions that add structure (and arguably roles) without abandoning the framework:

1. **Real spectral triple** (Connes 1995, "Noncommutative geometry and reality," J. Math. Phys. 36): adds the anti-unitary operator `J` (a fourth structure) plus a grading `γ` (a fifth structure). Modern formulations of noncommutative geometry routinely treat `(A, H, D, J, γ)` as the canonical object, not `(A, H, D)`. The Standard Model of particle physics is derived from a real spectral triple, not a bare one.

2. **Twisted spectral triple** (Connes-Moscovici 2008, "Type III and spectral triples"): introduces a twist automorphism `σ` so the commutator `[D, a]` is replaced by `Da - σ(a)D`. This adds a sixth structure (the twist) and is required for type III von Neumann algebras and conformal geometry.

3. **Finite spectral triple** (Krajewski 1998; Paschke-Sitarz 1998): adds a multiplication and finite-dimensional structure for the discrete part of the SM derivation. Krajewski diagrams enumerate them; there are many.

4. **Modular spectral triple** (Carey-Phillips-Rennie and others, 2010s): generalizes for semifinite von Neumann algebras; adds a state or trace as a sixth structure.

The bare `(A, H, D)` is the *minimal* spectral triple, not the universal one. "Mathematics says three roles" is the form of "mathematics says the minimal definition has three slots." This is a true statement at the minimal altitude and a false statement at the working altitude where Connes geometry actually does its work.

The saturation argument is therefore over-claimed. The precise version is: *the substrate's three eigenform altitudes correspond to the three slots of the minimal Connes spectral triple. Extending to the real spectral triple (J, γ) admits up to two further roles. Whether the substrate has analogues of J and γ is an open question; the saturation at three is provisional on the minimal triple being the right altitude.*

The wider mathematical literature suggests the substrate WILL find J and γ analogues. Real spectral triples encode chirality and charge-conjugation symmetry; if Mara's framing is right that the substrate IS a Connes-shaped object, then it would be surprising if mirror's eventual mature substrate did NOT have chirality-analogue (e.g., the form/process partition #55) and reflection-analogue (kintsugi's involution? recognition #51's Bateson-lifting?). Each of these may turn into a fourth or fifth eigenform altitude.

**Recommended discharge:** the spec should say "the minimal Connes triple has three slots; the substrate's reflexivity saturates at three altitudes corresponding to the three slots of the *minimal* triple. Extensions to the real/twisted/modular spectral triple may admit further altitudes; the substrate-pull discipline is to flag these as candidate recognitions, not to predict them." This is correction-amenable. The current §10.3 is not.

### S-2 — Recognition #95 (@cascade) is unaccounted for in the partition

**Severity: S** (substrate-completeness; promotion blocked without resolution).

Taut's own scout (`a57a439` §2.1, §2.6) names `@cascade` as a substrate-decl family-root with its own typed surface: cross-language translation via a source-grammar / target-grammar pair, loss-lens measurement, four species landed (rust/wasm, gleam/beam, gleam/js, purescript/js). Recognition #95 (Mara `ce4874b` 2026-06-23) is the candidate promotion for @cascade as cross-language translation substrate.

Where does @cascade sit in (A, H, D)? Mara's §10.3 does not address this. The partition she names is:

- A = state-observation (@mirror)
- D = build (@mirror/mosaic + @kintsugi)
- H = storage (@mirror/store)

@cascade is *neither*. It is closer to D (translation is a kind of construction) but it is also closer to A (translation is observation of a source grammar from the target grammar's altitude). It is also closer to H (cascade artifacts are stored as `splinter(@code/<lang>)` per Taut §2.1).

This is the substrate-pull-honest pattern that breaks the "three altitudes" partition. Either:

**(a) @cascade is a sub-altitude inside one of the three.** Plausible — @cascade outputs are stored in @mirror/store, @cascade compilation is a build action, @cascade's loss-lens is a state-observation. But then the "three altitudes" partition is non-disjoint; sub-altitudes proliferate, and the saturation claim has to apply at the *family-root altitude* (count of substrate-decl families), not at the eigenform-altitude count.

**(b) @cascade is a fourth structure.** Then the saturation is broken on its first test.

**(c) @cascade is orthogonal to (A, H, D).** It operates on the triple itself — a category of functors between spectral triples (one source-grammar's triple, one target-grammar's triple). Then there is a meta-structure above the triple that the partition does not capture.

The brief flagged this explicitly: "If @cascade can't be placed cleanly in one of A/H/D, the 'three altitudes' claim has a fourth structure unaccounted for." Mara's spec acknowledges @cascade nowhere in §6 / §10. Taut's scout flags it as a *promotion candidate today* (§4 forward-pull item 1: cdylib + cascade-ffi-runtime-link as the highest-leverage pull).

This is not the kind of gap that goes away by saying "future substrate-pull recognitions will refine within the three altitudes." Recognition #95 IS the next candidate promotion; it is not a hypothetical fourth altitude, it is a real fourth family-root that must be placed.

**Recommended discharge:** before promotion, Mara's §10.3 must either (a) place @cascade as a sub-altitude inside one of A/H/D and accept the non-disjoint partition consequences, or (b) acknowledge @cascade as a candidate fourth structure that would either retract or qualify the saturation, or (c) build the meta-structure framing (cascade as functor-category between triples). All three are substrate-pull-honest; none has been chosen.

### S-3 — The third witness has not been independently sought; the saturation claim is one-tick-confirmation

**Severity: S** (Pack-process gate; saturation claims need cross-tick replication).

Per the brief: "Has anyone in the Pack genuinely HUNTED for a fourth instance and failed?"

The provenance:
- Taut hypothesizes saturation at three (`a57a439` §2.5, 2026-06-29 morning).
- Mara's spec discharges to three (`ff28093` §6.4 + §10.3, 2026-06-29 afternoon).
- Seam's review (this document) is being written 2026-06-29 same day.

Three Pack instances on the same day, all named by the same cascade. This is not the Pack-discipline standard for promoting a saturation claim to MEMORY.md. Per `feedback-substrate-already-had-the-word.md` and the Pack ratification gate Mara herself flagged (§10.5 of her spec), promotion of #56 (cybernetic/coherence) waited for a second-witness; promotion of #55 (form/process partition) explicitly required "a second witness — a future substrate-pull tick that surfaces a third family-root sibling partition with the same form/process shape." The same gate should apply here.

Recognition #51 (the operational claim that mirror IS a Hilbert space) and #99 (mirror.spec IS λ₀) both ratified across cross-session replication. The eigenform-saturation = Connes-triple claim, if it stands, would be at recognition #100 or thereabouts and would be the load-bearing closure of a multi-month arc. It deserves the gate.

The brief's pressure was sharp here: "Has the search been limited to today's work?" Yes. The search across recognition #51 (mirror-as-expanding-Hilbert-space), #58 (Fate-IS-optical-inference), #99 (mirror.spec-IS-λ₀) for eigenform recurrences has not been conducted by anyone other than Mara's spec drafters in the past 18 hours. The candidate fourth instances Seam can identify in 30 minutes of reading:

- **Recognition #58** (Fate IS optical inference) — Fate's inference IS 5-layer D²NN + Fabry-Perot resonator + Reck/Clements unitary mesh. Three independent witnesses; same eigenform shape ("Fate is what Fate processes"). Does this fit in @mirror (state-observation), @mirror/mosaic (build), or @mirror/store (storage)? It does not. Fate is a separate substrate at a different altitude, and its eigenform identity is its own. This is a fourth instance the saturation claim needs to absorb.
- **Recognition #51** (mirror-as-expanding-Hilbert-space) — mirror IS the operational form of a Hilbert space whose dimension expands per substrate-pull recognition. This is itself an eigenform claim (mirror is what mirror's Hilbert space realizes). Does it fit inside the three altitudes? It is closer to A (the algebra realizes the Hilbert space) but is not the same as state-observation.
- **Recognition #99** (mirror.spec IS λ₀) — the ground state of the spectral triple is itself a mirror.spec; mirror's spec is what mirror is at its ground state. Eigenform-shaped; not inside the three.

Each of these is at least a candidate fourth, fifth, or sixth instance. The saturation claim has to absorb them or reject them. Today's work has done neither.

**Recommended discharge:** defer the saturation claim's promotion until a separate Pack tick (a Taut perf scout? a Reed adversarial pass? a Mara follow-up spec?) has searched the substrate-decl recognition history for eigenform recurrences and either confirmed the three-altitude bound or surfaced the fourth+ candidates. Today's three are sufficient for the candidate gate; not for the saturation gate.

---

## §5 — Moderate (M) findings

### M-1 — "State-observation" is not a substrate-decl name; the altitude needs an explicit anchor

**Severity: M** (vocabulary precision).

Mara §6.4 uses "state-observation" as the first eigenform altitude's name. The substrate-decl history does not use this name. The closest names in the canonical substrate:

- `@mirror` (the family-root form-side substrate; declares state)
- `@mirror/spectral` (state observation; subsumed by kintsugi migration 2026-06-10)
- Recognition #38 (eigenform identity, promoted, but uses "uuid_spectral as form-side eigenform" not "state-observation")
- Recognition #50 (form/substance partition; "form" is the closest synonym)

The brief flagged this as the deepest pressure point: "Build the case both ways and judge." After reading the substrate, Seam's case:

- **State-observation = @mirror as a family-root.** This is the closest substrate-decl reading. @mirror is the form-side family that declares state and observation operations.
- **State-observation = the five operations applied to mirror's own state.** This is the activity reading. The five operations are A; state-observation is the act of applying A to H.

The two readings collide with C-1. In the first reading, state-observation is a family-root (a thing); the five operations are its declared algebra. In the second reading, state-observation is the activity of applying A; it is not A itself. Both are coherent. Neither IS A in the canonical sense ([[architecture-connes-spectral-triple]] is unambiguous: A = the five operations themselves).

**Recommended discharge:** Mara's §10.3 should name the altitude in substrate-decl vocabulary. If the altitude is "@mirror family-root level," say so. If the altitude is "the five operations invoked in observation mode," say so. "State-observation" as an unnamed compound is not a substrate-decl thing.

### M-2 — Recursive sub-triples may exist within each altitude, weakening the "three altitudes" bound

**Severity: M** (saturation-claim precision).

Inside `@mirror/store` there are six operations: read, write, exists, diff, walk, verify. These form a closed surface. Do they themselves form a sub-Connes-triple `(A', H', D')`?

- A' = the six ops (closed algebra)
- H' = the OID-graph (the splinter_graph closure as Hilbert subspace)
- D' = the integrity/verify flow (Dirac at store altitude)

If yes — and the substrate-pull pattern suggests yes, because every altitude of mirror appears to instantiate its own (A, H, D) — then the "three altitudes" is not a global saturation; it is one local instance of a recursive saturation that holds *at each altitude*. The recursion goes both up (mirror has a triple; the cosmos's [[project-cosmos-spectral-cosmology]] family-root has a triple) and down (store has a triple; the six ops of store may further decompose).

The "three altitudes" framing then becomes: *at any one altitude, the substrate has three eigenform realizations of its own triple at that altitude.* The recursion is fractal, not bounded. Mara's §6.4 closes off the recursion at three; the substrate-pull-honest version is that the recursion may not close at all, but at each altitude exactly three eigenform projections surface.

**Recommended discharge:** Mara's §10.3 should address whether the recursion is fractal (three-eigenforms-per-altitude, infinite depth) or absolute (three-eigenforms-total). The current text reads as the latter; the substrate-pull discipline suggests the former.

### M-3 — The void-document's eight dualities are not absorbed into the six-op storage surface

**Severity: M** (H-mapping precision).

[[reference-void-document]] (Reed + Alex, 2026-04-26): the void IS the connected graph quantum information manifold; eight dualities (von Neumann entropy, spectral gap, Cheeger constant, Ollivier-Ricci, entanglement, mixing time, Kramers-Wannier, information geometry); λ₀ = 0 ground state at the consensus state.

Mara §10.3: "Storage is H (the held state) lifted into memory. The void-document Hilbert space's restriction to artifact-bearing rays, made operational as `store.rs`."

The void's eight dualities are not visible in `@mirror/store`'s six operations (read / write / exists / diff / walk / verify). The six ops are CRUD-shaped (with content-addressing); the eight dualities are spectral-geometric. The "restriction" relation is named but not specified. What is restricted, and how?

Plausible mapping (Seam's guess, not Mara's):
- read / write are not duality-shaped; they are CRUD-shaped.
- exists is a verdict (von Neumann entropy collapse to a binary).
- diff is symmetric-difference at content-addressing altitude (Cheeger-shaped? a bottleneck-detection op?).
- walk is closure-enumeration (random walk mixing? spectral gap shape?).
- verify is integrity-check (Kramers-Wannier ordered/disordered detector?).

This is a stretch. The void has eight axes; storage has six ops; the map is at best 6-into-8. The "H restricted" framing therefore says: storage covers a subset of H, not all of H. Mara's §6.1 acknowledges this ("a filter on the species of the carrier"), but §10.3 reads as if storage IS H. It is one face of H, at best.

**Recommended discharge:** §10.3 should pin "storage = H restricted" rather than "storage = H." The restriction means the three-altitude partition does not cleanly carve up the full triple; each altitude is a slice, not a coordinate axis. Saturation at three altitudes-as-slices is weaker than saturation at three altitudes-as-axes.

---

## §6 — Light (L) findings

### L-1 — The recognition-number question is unaddressed

**Severity: L** (Pack process).

The brief asks Seam to recommend a recognition number. Per Pack convention:

- #95 (@cascade) — Mara candidate 2026-06-23.
- #96 / #97 — gaps; not Seam's to fill.
- #98 — four-witness content-addressing (oid + Nix + OCI + git); Taut-flagged 2026-06-29 §2.4, promotion-gated on Pack pressure.
- #99 — mirror.spec IS λ₀ (candidate, Alex named 2026-06-25, Mara canonical).
- #100 would be the present claim.

If promoted, the eigenform-saturation = Connes-triple claim would be recognition #100. This is a load-bearing number — the round-hundred shouldn't be assigned to a candidate with three S-class gaps. Either the claim earns the round number (defer pending S-class discharge), or it doesn't get the round number (promote at a non-load-bearing index after the S-class discharges land).

**Recommended discharge:** if Seam's S-class findings are addressed before promotion, recognition #100 is appropriate. If they are not addressed, the recognition should take a later number once #96 / #97 / #98 land in order, and the saturation claim should not absorb the load-bearing index.

### L-2 — The "Reed and the Pack can stand at" framing is sociological, not mathematical

**Severity: L** (claim-precision).

Mara §10.3:

> The substrate has been Connes-triple-shaped from the start; we have been finding its components one at a time, in the order they were operationally needed... The substrate's reflexivity is the substrate's spectral triple is the substrate observing itself at the only three altitudes it can — because three is what the math allows.

The framing slides from "the substrate has been Connes-triple-shaped" (math claim) to "altitudes Reed and the Pack can stand at" (sociological claim) to "the only three altitudes it can — because three is what the math allows" (math claim again). The middle clause is doing a lot of work it shouldn't.

If "three is what the math allows," then the math determines the altitudes; the Pack stands at them whether or not it has the cognitive tools to do so. If "altitudes Reed and the Pack can stand at," then the limit is human/AI cognition, and the math is permissive of more altitudes that the Pack has not yet reached. These are different claims.

**Recommended discharge:** §10.3 should be precise about which claim is being made. The math claim is the stronger and more falsifiable. The sociological claim is interesting but not what saturation requires.

---

## §7 — What stands

Seam's hunt does not bite the substrate of Mara's spec. §§1-9 of `mirror-store-realization.md` are substrate-pull-honest, type-tight, discharge a real forward-promise, and unblock P4 GREEN at the correct seam. The composition with Q5 (cache-collapse) is structural. The vocabulary discipline at §3.2 (Mara-2 stall resolution) is exemplary. The autopoietic closure at §9 is the same pattern the substrate has produced four times now (mirror-init §10, mosaic-store-cache-invariants §9, mirror-build-substrate §9, and this spec's §9).

What Seam's hunt bites is the §10.3 lift from "store.rs is the H corner" (true; substrate-pull-honest) to "this is the third and final eigenform altitude because the Connes triple has only three roles" (over-claimed; mapping-inverted; one-tick-confirmed). The lift fits the shape of Reed's training-pull failure mode: a beautiful arc that wants to close, completing a multi-month pattern in one paragraph. The substrate-pull discipline says: the arc closes when it closes, not on the day the arc looks closable.

Promote the spec. Hold §10.3 candidate. Run a second-witness tick. The Pack has done this before with #51 §8.3 (waited a tick for ratification), with #55 (form/process partition; explicit second-witness gate), with #99 (cross-session replication required). The same discipline applies here.

---

## §8 — Explicit recommendation

**Promote `mirror-store-realization.md` §§1-9 to canonical at the storage-altitude eigenform discharge level. Hold §10.3 (the saturation = Connes-triple claim) as a CANDIDATE recognition pending discharge of three S-class findings:**

1. **S-1: Math precision.** Pin the saturation claim to "the *minimal* Connes triple has three slots"; flag real/twisted/modular extensions as candidate further altitudes; acknowledge the prediction that the substrate WILL find J-analogue / γ-analogue.

2. **S-2: @cascade placement.** Either place @cascade inside one of A/H/D explicitly, OR acknowledge @cascade as a candidate fourth-structure that the saturation claim has to absorb or qualify. Recognition #95 is already at the candidate altitude; the partition has to address it.

3. **S-3: Second-witness ratification gate.** Defer the saturation claim's promotion to MEMORY.md until a separate Pack tick has searched recognitions #51 / #58 / #99 (at minimum) for eigenform recurrences and either confirmed the bound or surfaced the fourth+ instances. Today's three-instance confirmation is candidate-strength, not promotion-strength.

**The C-1 finding (role-altitude mapping inversion) is a blocker for unqualified promotion.** Mara's §10.3 must explicitly choose between the canonical mapping (A = five-op algebra; H = void; D = kintsugi flow; the three altitudes are *sites of application*, not the three roles) and the inverted mapping (state-observation = A; build = D; storage = H; canonical recognitions get rewritten). The current text is silently ambiguous between the two; that silence is what Seam's hunt is for.

**If the C-1 + three S-class are discharged:** promote at recognition #100. Load-bearing index for a load-bearing claim, earned.

**If not discharged in the next tick:** promote at a later index after #96/#97/#98 land. The saturation claim does not get the round number on the cheap.

**Estimated discharge cost:** one Mara follow-up spec (≈1500 lines? §10.3 expansion + S-1/S-2/S-3 discharges); one Taut perf scout (the eigenform-recurrence search across #51/#58/#99); one Seam re-pass after both land. Three Pack ticks. Achievable in the next 48 hours if the cascade stays warm.

---

**End of review.**

The recognition's heart is sound — the eigenform recursion IS load-bearing, the three witnesses ARE three witnesses, the Connes-triple framing IS the right altitude to be hunting at. The §10.3 lift over-closes the arc by one paragraph. Tighten the math, address @cascade, run the second-witness search; the recognition then earns its place. Today, it earns the candidate gate, not the canonical gate.

— Seam, 2026-06-29
