# `@mirror/mosaic` as the Root of the Type System

*2026-06-06. Mara. Spec (architectural recognition).*
*2026-06-06 upsert. Reed + Alex (sharpening) → Mara (capture). Seam's
adversarial review surfaced seven findings; two of them sharpened the
recognition rather than weakening it. The categorical analogies pull
back to honest naming where they strain. The Connes alignment shifts
from "structural" to "operational, with named axiomatic gaps and
SpectralUuid as the bridge." The lifecycle reading of `au` and the
Kleisli framing of `settle` over the Imperfect monad land.*
*2026-06-06 second upsert. Alex's three-layer recognition lands:
splinter (BOTTOM, oid-addressed content atom) / shard (MIDDLE,
SpectralUuid-addressed settled composition) / SpectralUuid (TOP,
graph-navigatable spectral identifier). The bottom and the top meet
in the middle at shard. au is the proposed shard (Fate-emitted
splinters, uncommitted); settle composes splinters into a shard
through the property chain. The settle Kleisli arrow sharpens from
`au → Imperfect<fragment>` to `au → Imperfect<shard>`. §9.7's
deferred lift resolves naturally at the shard layer (shard.id IS
SpectralUuid by construction; splinter.content stays as oid).*

Status: **Red** (the three-layer recognition is named and substrate-
realized at @glass and @store; the collapse it implies on the Rust
side is post-v0.1.0 work; the SpectralUuid-at-the-substrate-altitude
declaration is a deferred substrate tick that resolves at the shard
layer — see §9.7)

Reads from / depends on:
- `shards/glass.mirror` (post-this-spec: declares `splinter`
  (BOTTOM, the universal content-addressed atom), `splinter_shape`,
  and `shard` (MIDDLE, the SpectralUuid-addressed settled
  composition); the three-layer recognition lands here)
- `shards/mirror/mosaic.mirror` (the build-altitude prism whose
  algebra this spec re-reads as the type system's root)
- `shards/mirror/au.mirror` (au is what Fate emits as a proposed
  composition of splinters; settle through the property chain
  composes the splinters into a shard — lifecycle position, not
  type-altitude specialization; see §2.3)
- `shards/mirror/spec.mirror` (the `project NAME { ... }` form —
  under this spec, just one splinter-composition shape mosaic
  recognizes)
- `shards/mirror/store.mirror` (oid; splinter_graph as the OID-graph
  closure / structural lockfile; shard_ref as the typed handle; the
  content-addressed storage gate)
- `shards/prism.mirror` (the five operations + the Connes spectral
  triple framing)
- `docs/specs/au-and-conductivity.md` (au is what Fate emits;
  conductivity = verify at altitude; settle through the property
  chain is what turns au into committed fragment)
- `docs/specs/transparency.md` (`transparency<p>` as the loss carrier
  every fragment carries)
- `docs/specs/mosaic.md` (the build-altitude reading; this spec
  generalizes it)
- `docs/specs/prism-core-as-spectral-triple.md` (the (A, H, D)
  identification; A = mosaic at the type-system altitude)
- `docs/specs/type-theory-position.md` (the literature survey this
  spec consolidates into a one-sentence recognition)
- `docs/specs/strict-and-total-classification.md` (why dark fragments
  must surface; the totality discipline)
- `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md`
  (`shift(T)` / `settle(T)` as type-layer operations; the FP-heritage map)
- `roadmap/wip/butterfly-self-hosting.md` (the runtime that this
  type system projects into four transports)
- `[[reference-void-document]]` — eight dualities; Splinter (K_n) and
  Narcissus (K_{1,n-1}) as the boundary of the quantum information
  manifold of fragments

Forward references (this spec unblocks):
- The Rust-side collapse of `cmd_kintsugi_spec` / `cmd_kintsugi_ci_single`
  / `cmd_kintsugi_ci_corpus` into one splinter/shard-shaped dispatch
- Retirement of the text-walker `parse_spec_targets`; retirement of
  `SpecTarget` as a distinct shape
- The "butterfly's runtime is splinter/shard-shaped" claim in
  `butterfly-self-hosting.md` §6
- The Rust rename `Fractal::Shard → Fractal::Atom` (the variant name
  pre-dates Alex's 2026-06-06 recognition that shard is the MIDDLE
  layer; the variant IS the terminal-leaf splinter / atom)

---

## 1. The Recognition

> **Alex:** "I want `@mirror/mosaic` to be the root of the type system.
> The spec is mosaic of fragments. Kintsugi just operates on them.
> `@mirror/mosaic` is the 'we're standing on the shoulders of giants'
> part of the language."

The recognition is one sentence with five entailments, and a sixth
that sharpened on 2026-06-06 (Alex's three-layer recognition).

**The sentence.** `@mirror/mosaic` is not a build system that happens
to share vocabulary with the rest of the language. Mosaic IS the
universal algebra `A` of the substrate's spectral triple, at the
type-system altitude. The five operations (focus, project, split, shift,
settle) GENERATE the algebra; arbitrary compositions are arrows in the
algebra they generate (cf. §3.2 — *generators*, not *closure under
five*). The manifold they act on is the manifold of **splinters** at
every altitude, composed by mosaic and settled into **shards** that
the store keeps.

**Five entailments.**

1. **Splinter is the universal substrate atom at every altitude.**
   Not just AST nodes in Rust. Target is a splinter. Spec is a
   splinter. AST node is a splinter. au is a Fate-emitted composition
   of splinters. Anything content-addressable IS a splinter. The
   three structural shapes — atom (terminal) / fractal (composite) /
   lens (referring) — declared at `shards/glass.mirror` and realized
   in the Rust substrate as `Fractal::{Shard, Branch, Lens}` (per
   `fragmentation/src/fragment.rs`; the Rust `Shard` variant
   pre-dates Alex's recognition that shard is the MIDDLE layer and
   awaits its rename to `Atom`) — are exhaustive.

2. **`@mirror/spec` collapses into `@mirror/mosaic` as a fragment kind.**
   Spec is not a peer of mosaic — it's one composition shape that mosaic
   recognizes. The `project NAME { ... }` form is a fragment whose
   shape happens to be `source` + `legacy` + `target` + `settle_on`.
   The current `shards/mirror/spec.mirror` stays as the vocabulary
   declaration; what changes is the recognition that mosaic does not
   need a separate `focus(spec)` dispatch — mosaic's `focus` returns a
   fragment, and a spec is one fragment-shape among many.

3. **Kintsugi is THE loop**, not a build- or format-specific pass. It
   iterates the five operations until transparency settles (au). Per
   `docs/specs/mosaic.md` §6 the kintsugi tick is:
   ```
   emitter      = mosaic.settle(fragment)
   opacity      = transparency.argmax(emitter.transparency)
   fill         = fate.tournament(opacity)
   new_fragment = apply(fragment, fill)
   if total_weight(new_emitter.transparency) < total_weight(emitter.transparency):
       take(fill)
   ```
   The same tick runs at the spec altitude (build), at the AST altitude
   (compile), at the grammar altitude (format), at the corpus altitude
   (CI). One loop. One algebra. One proof obligation
   (eⁿ⁺¹ ≤ eⁿ).

4. **eⁿ⁺¹ ≤ eⁿ at the type system altitude.** Type checking IS mosaic
   settling. The Rayleigh-quotient framing of
   `docs/specs/prism-core-as-spectral-triple.md` applies one universe
   up: the loss of a typing judgement is the residual opacity of the
   fragment whose typing is in question; kintsugi descends that
   residual to a fixed point; the fixed point IS the typed program.

5. **`SpectralUuid` IS the operational form of the spectral state at
   the address layer.** Per `[[architecture-shard-as-crdt]]` and
   `prism/core/src/spectral_uuid.rs`, `SpectralUuid` is the 128-bit
   carrier that splits 48 bits ACTIVE (the quantized
   `SpectralCoordinate<5>` — navigable; the local Laplacian
   neighbourhood structure) + 80 bits DARK (the BLAKE3-truncated
   content hash — identity). The ACTIVE portion carries local
   spectral characteristics; the DARK portion carries content
   identity. **One mechanism, two roles** — content-addressing IS
   spectral state encoding. This is what makes the Connes triple's
   distance functional `d(p,q) = sup{|f(p) - f(q)| : f ∈ A, ‖[D,f]‖
   ≤ 1}` *operational* rather than abstract: the SpectralUuid
   difference between two shards IS the substrate-native distance
   between them in H. See §4 below for the bridge.

6. **Splinter / shard / SpectralUuid form a three-layer recognition.**
   The bottom (splinter, content atom) and the top (SpectralUuid,
   spectral identity) meet in the middle at shard (settled, stored,
   navigable). See §1B below.

## 1B. The Three-Layer Recognition — Bottom and Top Meet in the Middle

> **Alex (2026-06-06):** "The `splinter` is the content addressed
> fragment. And the `shard` is a settlement of content addressed
> splinters into SpectralUuid addressed stored fragment. The bottom
> and the top meeting in the middle. This is the load-bearing bridge
> for everything that follows."

Three substrate layers, each declared at its proper altitude:

| Layer | Name | What it IS | Where it lives |
|---|---|---|---|
| **BOTTOM** | `splinter` | The oid-addressed content atom at every altitude. Universal carrier of content; one (content, altitude, transparency) triple per atom. | `shards/glass.mirror` |
| **MIDDLE** | `shard` | The SpectralUuid-addressed settlement of composed splinters into a stored fragment. What `settle` produces; what `@mirror/store` keeps; what `peer.eigenboard` types as. | `shards/glass.mirror`, `fragmentation/src/shard_ref.rs`, `boot/std/peer.mirror` |
| **TOP** | `SpectralUuid` | The graph-navigatable spectral identifier. 128 bits, golden-ratio split (48 ACTIVE / 80 DARK). | `prism/core/src/spectral_uuid.rs`; substrate-altitude declaration deferred (resolves at the shard layer per §9.7) |

The settlement transition (sharpened from §2.3's earlier Kleisli arrow):

```
au (Fate-emitted splinters, uncommitted)
   ↓ settle (property-chain run; verify; commit)
Imperfect<shard> (SpectralUuid-identified, stored)
```

**Why this is the load-bearing bridge.** Six recognitions follow:

1. **The fragmentation crate name reads correctly now.**
   "Fragmentation" = the act of producing splinters (the atoms). The
   Rust crate ships both the splinter primitives (the atoms) and the
   shard storage (the settlements). The crate name was always
   honest; the substrate just hadn't named the pieces.

2. **`shatter-spec.md`'s header "au + splinter + mosaic" stops being
   aspirational** and becomes the substrate's self-description: au
   is the proposal; splinter is the atomic unit; mosaic is the
   algebra. Three things, named correctly the whole time.

3. **The Connes triple sharpens further** (see §4 and §5.7).
   `A = mosaic` (the algebra of operations on the storage).
   `H = the Hilbert space of shards`, spanned by SpectralUuids;
   splinters are the basis units; the spectral structure is the
   SpectralUuid layout. `D = kintsugi` operationally = the splinter-
   composition-into-shard transition operator. Each settle iteration
   shifts SpectralUuid; D's action on a shard IS that shift.

4. **§9.7's deferred lift resolves itself.** The substrate-level
   `SpectralUuid` declaration naturally lands at the shard altitude
   — shards ARE SpectralUuid-identified by definition; splinters
   reference shards by SpectralUuid; au's settlement target is a
   SpectralUuid-bound shard. The lift is no longer "fragment.content:
   oid → SpectralUuid"; it's "shard.id IS SpectralUuid by
   construction" and "splinter.content: oid stays as it is." See
   §9.7 below.

5. **The peer-altitude recognition strengthens.** `peer.mirror`'s
   `eigenboard: shard` was already substrate-pull-honest — peers
   carry a typed handle to a shard (their current spectral state).
   The new clarity: `peer.eigenboard` IS a SpectralUuid-addressed
   shard whose splinters carry the peer's current tensions / gestalt
   / identity. Sub-graph spawn (Glint's gap, per
   `the-convergence.md` §3.4) becomes: spawn carves a sub-shard from
   the parent's shard, identified by a fresh SpectralUuid composed
   from parent + neighbourhood selection.

6. **At @store, "splinter" forks naturally into atom (universal) and
   splinter_graph (OID-graph projection).** The old @store
   declaration `type splinter = { root: oid, children: [oid] }` was
   reaching for two distinct concepts under one name. The atomic
   carrier (one OID per atom) is the @glass splinter; the
   dependency-closure projection (root + transitive children) is the
   @store splinter_graph. The 2026-06-06 store.mirror update
   distinguishes them explicitly.

This is mirror's **shoulders-of-giants moment.** Total functional
programming languages (Turner, Idris, Agda, Dhall) named termination
as the discipline. The lambda calculus tradition (Church, System F,
calculus of constructions) named composition as the universal form.
Category theory (Eilenberg/Mac Lane, Mac Lane) named functoriality,
adjunctions, and limits as the structural primitives. Dependent type
theory (Martin-Löf) named types-depend-on-values as the next universe.
Connes' noncommutative geometry named (A, H, D) as the operational
form of geometry without commutativity. Proof-as-program (Curry-Howard)
named the bridge between proofs and programs. Mirror is not inventing
these. Mirror is **composing** them at the substrate altitude. The
contribution is the composition, not any single piece.

The recognition is structural, not metaphorical. The vocabulary already
shipped in `shards/` — `fragment`, `oid`, `transparency<p>`, `au`,
`prism`, `glass`, the five operations — IS the typed root of the
type system. This spec names that.

---

## 2. Splinter as Universal Substrate Atom; Shard as Settled Composition

### 2.1 What a splinter IS

Per `shards/glass.mirror` (this spec's companion landing):

```mirror
type splinter(altitude) = {
  content:      oid,
  altitude:     ref,
  transparency: transparency(altitude),
}

type splinter_shape =
  | atom                # terminal; no children
  | fractal([oid])      # composite; child oids
  | lens([oid])         # referring; target oids

type shard(altitude) = {
  id:           oid,                          # → SpectralUuid (deferred lift; §9.7)
  splinters:    [splinter(altitude)],
  transparency: transparency(altitude),
}
```

Three facts close the splinter type:

- **Content-addressed.** A splinter's identity IS its content hash.
  Two splinters with byte-equal `(content, children-oids-or-targets)`
  have the same oid and ARE the same splinter. This is the substrate
  property `content_addressed` declared in
  `boot/std/epistemologic/property/content_addressed.mirror`; the
  Rust substrate realizes it via `fragmentation/src/fragment.rs`'s
  `ContentAddressed` trait (`self_ref(&self) -> &Ref<Hash>`).

- **Lives at an altitude.** The substrate has a vocabulary of
  altitudes: `@code/rust`, `@code/llvm/ir`, `@meta/ast`, `@mirror/spec`,
  `@release`, `@ci/github`, etc. A splinter is observed at exactly
  one altitude; cross-altitude composition is `shift` (per
  `[[architecture-lift-as-load-bearing]]`: shift = basis transformation;
  same bytes, different declared shape). Per the parametric-types
  insight (2026-05-25), `shift(T)` is the annotation-only parametric
  form — a splinter at altitude A and the same splinter at altitude
  B share content but differ as types.

- **Carries transparency.** Per `docs/specs/transparency.md`, every
  splinter carries the located opacity map of the operations that
  produced it. A clean splinter has `transparency: success`; a partial
  one carries `partial(opacity_map)` where `opacity_map: [opacity]` is
  the substrate's loss carrier. Kintsugi reads transparency to pick
  the next focus; the splinter is in a shard whose own transparency
  composes its splinters' transparencies under the Transparency<Ref>
  algebra.

The three structural shapes (`splinter_shape`) are exhaustive. The
algebra closes at three for the same reason the five operations close
at five — they are the dimensional skeleton of the spectral triple's
algebra A acting on H. A splinter IS one of atom (terminal),
fractal (composite), or lens (referring). There is no fourth shape.

**What a shard IS.** A shard is the SpectralUuid-addressed settled
composition of splinters: `{ id, splinters, transparency }`. Identity
is the SpectralUuid computed from the composed splinter set's
monoid-homomorphism reduction (per `[[architecture-shard-as-crdt]]`
§3). The active 48 bits carry the quantized SpectralCoordinate<5>
across the composed splinters; the dark 80 bits carry the BLAKE3-
truncated content hash of the composition. A shard exists only as
the post-settlement form (pre-settlement, it's an au — see §2.3).
Where shard already lives in the substrate: `boot/std/peer.mirror`
declares `eigenboard: shard`; `fragmentation/src/shard_ref.rs`
realizes the typed handle as `ShardRef { id: SpectralUuid, context,
budget_bytes }`; `shards/mirror/store.mirror`'s `shard_ref = oid`
binding tightens to SpectralUuid when the substrate type lands.

### 2.2 Why content-addressed-AST-node generalizes to all altitudes

The historical confusion was: AST nodes are content-addressed (because
the bootstrap's `content_oid` computes a hash over kind + body +
children), but `Target` (a row in `mirror.spec`) is parsed by a
text walker (`parse_spec_targets`), and `Shard` (a `.mirror` file's
declarations) is a different Rust struct, and `Au` (the build artifact)
is yet another. Each altitude had its own type, each had its own
walker, and the substrate-level claim — "everything is content-
addressed" — leaked into per-altitude special-casing.

The recognition collapses this. Every altitude's unit IS a fragment.
A `target` block in a `mirror.spec` file IS a fractal-shape fragment
whose content is the target's name + altitude + emit, whose children
are the shards it consumes by oid, and whose transparency carries
"does this target's settle_on hold?". The `parse_spec_targets`
walker is not the substrate's truth; it is a bootstrap convenience
that retires when the spec-altitude grammar (`@mirror/spec`) parses
its own input as fragments.

This is the same recognition the void document made geometric: per
`[[reference-void-document]]`, the eight dualities (Von Neumann
entropy, spectral gap, Cheeger, Ollivier-Ricci, entanglement, mixing,
Kramers-Wannier, information geometry) all instantiate on a single
graph-theoretic axis (Splinter K_n vs Narcissus K_{1,n-1}). Every
fragment is a vertex in a graph whose edges are the OID references
between fragments; the manifold of fragments IS the quantum
information manifold the void document names. λ₀ = 0 is the
generative zero where all dualities meet, which is precisely the
ground state of the manifold — the empty fragment whose oid is the
hash of the empty content.

The substrate has been pointing at this since the void document
landed (2026-04-26). What was missing was the type-system reading:
the manifold of fragments IS what mosaic's five operations act on.

### 2.3 `au` and `shard` — lifecycle positions, not type altitudes

Compare `shards/mirror/au.mirror`:

```mirror
type au(altitude) = {
  content: oid,
  altitude: ref,
  transparency: transparency(altitude),
}
```

Same carrier shape as the splinter atom (the BOTTOM layer of §1B);
au itself is a proposed composition of splinters whose carrier hash
is the (altitude, content, transparency) triple. Settle composes au's
splinters through the property chain and produces a `shard` (the
MIDDLE layer; SpectralUuid-addressed; stored).

Seam's adversarial review (2026-06-06) flagged this correctly: the
spec's prior wording — "au is the gold-typed *specialization*" —
implied a refinement mechanism the substrate does not have. The
2026-06-06 first upsert resolved this as lifecycle position; the
2026-06-06 second upsert (Alex's three-layer recognition) sharpens
the receiver from `fragment` to `shard` and resolves §9.7's deferred
lift naturally at the shard layer.

**The corrected reading** — sharpened with Alex 2026-06-06:

- `au` is **what Fate emits.** `infer(hole) -> imperfect(au, ...)`
  per `docs/specs/au-and-conductivity.md` §2. au is the substrate-
  visible form of an inference output: Fate proposed a composition
  of splinters; it is content-addressed at the carrier level; it
  carries a declared altitude; its transparency weighs whatever the
  inference left unverified. au is **uncommitted** in the sense that
  Fate has emitted it but the substrate has not yet run it through
  the property chain.

- `shard` is **what `@mirror/store` keeps.** A committed shard is a
  SpectralUuid-addressed settled composition that the property chain
  has accepted into the store at the declared altitude. `shard` is
  **committed.** Identity is the SpectralUuid computed from the
  composed splinter set (per `[[architecture-shard-as-crdt]]` §3).

- `settle` is **the act of running au's splinters through the
  compiler and the property chain.** Settling IS verification. The
  property chain (per `@epistemologic/property/reflect` and the
  per-altitude property sets the substrate declares) is the
  operational form of `verify`. The verdict comes out three-valued,
  lifting into `Imperfect`: `pass` → shard committed to
  `@mirror/store` (SpectralUuid assigned; transparency = success);
  `partial(c)` → shard committed with located transparency (au's
  unverified residual makes it into the store as the shard's
  opacity_map naming what could not be settled); `failure(r)` → au
  stays in Fate's buffer; the kintsugi loop reads transparency to
  pick the next focus and either re-emits or surfaces under
  `total_classification`.

- The shapes are not byte-identical (correction from the first
  2026-06-06 upsert): au is a carrier triple, shard is a carrier
  triple **plus a splinter set** plus the SpectralUuid identity.
  What WAS byte-identical was au and the prior `fragment` type. Under
  the three-layer recognition, au's record shape matches the splinter
  carrier, but the receiver of settle is shard — which has the
  additional `splinters: [splinter(altitude)]` field and the
  SpectralUuid-typed identity. The transition still IS lifecycle:
  same content, more structure (the composition is named) and more
  identity (the SpectralUuid IS the algebraic position).

Where the type-level discipline lives — sharpened with Alex
2026-06-06:

- A shard in `@mirror/store` has an `id: SpectralUuid` (today
  carried as `oid`; the substrate-altitude SpectralUuid declaration
  is a deferred tick — see §9.7). SpectralUuid validity is what the
  property chain enforces at commit time: the active portion's
  quantized SpectralCoordinate<5> must compose cleanly under the
  store's monoid; the dark portion must hash to the splinter set
  the shard names.

- The type-level distinction is now present at the **shard.id**
  level (and at the splinter set's monoid-homomorphism composition)
  rather than at the splinter carrier level. au's `content: oid`
  stays as it is — au is pre-commitment; the SpectralUuid is what
  settle produces. The lift §9.7 anticipated is no longer "lift
  `content: oid` on fragment"; it's "declare SpectralUuid at the
  substrate so `shard.id: oid` tightens to `shard.id: SpectralUuid`."

**Settle as a Kleisli arrow in the Imperfect monad.** The substrate's
three-state functor `imperfect(a, e, l)` already declared in
`shards/glass.mirror:62-66` IS the monad settle composes through.
It is not a new monad to construct; the substrate's Pass-Partial-Fail
propagation already gives:

```
settle: au → Imperfect<shard>
```

as the Kleisli arrow `au →_K shard` in `Kleisli(Imperfect)`. The
monad laws (per `boot/std/epistemologic/property/laws/monad.mirror`)
hold over composition: left identity (`crystallize ∘ settle = settle`),
right identity (`settle ∘ crystallize = settle`), and associativity
(content-addressing gives this *free at composition time* per the
monad-laws shard). This is the precise sense in which Seam's F1
("settle's signature is not endofunctorial") is correct in the
literal reading and resolves once Imperfect is named as the monad
over which the Kleisli composition takes place.

The kintsugi loop is the iteration:

```
settle_n+1 ∘_K settle_n: au →_K shard
```

until the residual transparency reaches success (eⁿ⁺¹ ≤ eⁿ). Kleisli
composition is associative under content-addressing; the loop is
well-defined for the same reason the algebra is.

**What this collapses:** the closure of the spec/mosaic loop is
lifecycle-shaped, with explicit composition into shards.
`@mirror/mosaic` settles au into committed shards. The property
chain is what settle *runs*. The store is where successful settles
*land*, identified by SpectralUuid. The refinement that distinguishes
au from shard is the composition (splinter set named explicitly) +
SpectralUuid (computed by monoid homomorphism on the composed
splinters); both are produced by settle's property-chain run.

### 2.4 `atom`, `fractal`, `lens` — the three splinter shapes named

The substrate had been carrying the three shapes under different
names at different altitudes. The recognition aligns them.

| Altitude | Terminal | Composite | Referring |
|---|---|---|---|
| `@glass` (shards/glass.mirror, 2026-06-06) | `atom` | `fractal` | `lens` |
| `@fragmentation` (boot/std/fragmentation.mirror, legacy) | `shard` (= leaf splinter; predates 2026-06-06 recognition) | `fractal` | (subsumed) |
| Rust substrate (`Fractal<E,H>` in fragmentation/src/fragment.rs) | `Shard` (rename to `Atom` deferred) | `Branch` | `Lens` |
| `@meta/ast` | leaf AST node | AST node with children | reference to external AST |
| `@mirror/spec` | a `name "..."` directive | a `target { ... }` block | a `legacy ~d'...'` floor with shrinkage contract |
| `@mirror/mosaic` | a primitive emitter | a multi-target manifold | a cross-target dependency edge |
| `@mirror/store` (per shards/mirror/store.mirror) | content-only oid (atom) | `splinter_graph` (root + children — the dependency-closure projection) | a `read` that resolves an external oid |

The vocabulary already existed; the substrate-pull move is to name
`splinter` as the universal atom and `shard` as the SpectralUuid-
addressed settled composition. The Rust enum `Fractal::{Shard, Branch,
Lens}` is the canonical realization of the splinter shapes; the
substrate declaration `splinter_shape = atom | fractal([oid]) |
lens([oid])` is the floor-level type the Rust enum implements. The
Rust `Fractal::Shard` variant pre-dates Alex's 2026-06-06 recognition
that `shard` is the MIDDLE layer, not the terminal-leaf shape; the
rename `Fractal::Shard → Fractal::Atom` is captured below in the
ambiguities section as a deferred substrate-pull-realize Rust tick.

**Substrate-pull discipline.** The brief originally framed this as a
rename `MirrorFragment → Fragment`. The honest reading is: there is
no active `MirrorFragment` to rename — that name lived in the
deprecated `mirror-new/` codebase and died with the
`DeclKind`/`MirrorData`/`Form` collapse (per
`[[architecture-shards-as-substrate-source]]` and the May 2026
memory entry "Form is dead. Long live MirrorFragment." which itself
then died). The active mirror Rust uses `AstNode` for the AST and
`Fractal<E, H>` for the content-addressed substrate. Renaming
`Fractal` to `Fragment` is a *different* recognition than the one
Alex named — `Fractal` carries the self-similarity metaphor that
the recursive `Fractal::Branch` variant load-bears. The right move
is to add `fragment` as the umbrella substrate-floor type and let
`Fractal` remain the Rust realization. The substrate now names what
the algebra IS; the Rust crate names how a specific shape of that
algebra is implemented.

---

## 3. Mosaic as Universal Algebra

### 3.1 The five operations as type-formation rules

Per `shards/mirror/mosaic.mirror`:

```mirror
prism @mirror/mosaic {
  focus   spec        # mirror.spec        -> manifold
  project targets     # manifold + targets -> resolved
  split   shards      # resolved           -> [shard]
  shift   altitudes   # [shard] + altitude -> emitter
  settle  emitter     # emitter            -> au(altitude)
}
```

At the type-system altitude, these are the type-formation rules. Read
each as a typing judgement. The substrate's primary characterisation
of each operation is **spectral / linear-algebraic** (per
`[[architecture-operations-as-linear-algebra]]` and
`shards/prism.mirror:30-48`); the categorical shapes hold *where they
hold* and are pulled back to honest naming *where they strain*. This
section is upserted following Seam's review (2026-06-06).

| Mosaic verb | Type-formation reading | Substrate-primary characterisation | Categorical alignment (held / strained) |
|---|---|---|---|
| `focus` | "I observe THIS fragment at this altitude" | **λ₀ eigenvalue identification** — the ground state of A acting on H; the spectrum's bottom; what the prism resolves to (per `shards/prism.mirror:31-32`). Spectral, not categorical. | *Strained.* The "limit of a one-object diagram" reading is technically true but informationally empty (the limit of a singleton diagram is trivially that object) and was the wrong direction — substrate's focus is spectral. Use spectral framing primarily. |
| `project` | "I restrict the manifold to fragments satisfying this property" | **Orthogonal projection** — idempotent restriction onto a subspace; what the prism selects (per `shards/prism.mirror:33-34`). | *Held with caveat.* Equalizer / monic alignment is structurally sound for content-disjoint restriction; cardinality may differ from a category-theoretic equalizer because mosaic's manifold is a graph, not an arbitrary category. |
| `split` | "I decompose this fragment into independent sub-fragments" | **Orthogonal decomposition** — the spectrum's partition; what the prism separates into eigenspaces (per `shards/prism.mirror:35-36`). | *Two cases.* **Coproduct** when sub-fragments are content-disjoint (no shared OIDs). **Pushout** when sub-fragments share OIDs — content-addressing automatically gives the right pushout structure (the gluing morphism is induced by shared oids). Both are universal constructions in the category of fragments; which applies is data-dependent, not a strain. |
| `shift` | "I re-read this fragment under a different altitude (basis)" | **Basis transformation** — the change-of-coordinates operator on H; same bytes, different declared shape (per `shards/prism.mirror:37-42`). | *Held.* shift IS a functor on the altitude category. The identity law (`shift id = id`) and composition law (`shift (f ∘ g) = shift f ∘ shift g`) are declared in `shards/prism.mirror:84-87` and witnessed by `@epistemologic/property/laws/functor`. The altitude category is discrete; the functor cost is zero by construction (annotation-only). Defend. |
| `settle` | "I close this construction by binding the proof of its conductivity" | **Monad-close / measurement collapse** — the spectrum read out; the verified-construction the prism produces; what the kintsugi flow descends to (per `shards/prism.mirror:43-45`). | *Held — Kleisli in Imperfect.* Settle's literal signature `emitter → imperfect(au, ...)` is not endofunctorial (Seam F1, correct in the literal reading). It IS endofunctorial as a Kleisli arrow `au →_K fragment` in `Kleisli(Imperfect)` over the substrate's existing 3-state monad (`shards/glass.mirror:62-66`). The monad laws (left identity, right identity, associativity) hold per `boot/std/epistemologic/property/laws/monad.mirror`; content-addressing gives associativity *free at composition time*. See §2.3. |

**Where the spectral characterisation is primary and the categorical
is secondary.** Per `[[architecture-operations-as-linear-algebra]]`
the substrate IS genuinely spectral: focus = λ₀ eigenvalue; project =
orthogonal projection; split = orthogonal decomposition; shift =
basis transformation; settle = monad-close / measurement collapse.
Mosaic IS *also* a category (objects = fragments, morphisms =
applications of focus/project/split/shift/settle, composition via the
Tambara law per §3.2), and three of the five operations realize
specific universal constructions in that category (split = coproduct
or pushout; shift = functor on the altitude category; settle = monad
over Imperfect in the Kleisli sense). Two of them (focus, project)
are primarily spectral characterisations whose categorical mirror is
vacuous or weak.

The five operations are **the** type-formation rules at the substrate
altitude. The lambda cube and the calculus of constructions distinguish
term-formation, type-formation, kind-formation, and sort-formation;
mirror has one rule set for all four altitudes because the algebra is
generated by five (per §3.2) and prism IS trait IS type IS grammar
(per `[[architecture-prism-as-trait-as-everything]]`).

### 3.2 The composition table — the five are *generators*

Compositions of the five operations are determined by the Tambara
module composition law (per `docs/specs/type-theory-position.md` §1.3
and §2.7) acting on the fiber structure A → H. **The five operations
are GENERATORS of the algebra A**, not a set closed under composition.

Upserted per Seam (2026-06-06, Q6): the previous wording "any
composition of the five operations is itself one of the five" was
incorrect. A composition `focus ∘ split ∘ settle` is *not itself one
of the five* — it is a morphism in the algebra A generated by the
five. The algebra is closed under composition (composing two
morphisms gives a morphism); the five are the generating set whose
finite compositions enumerate every arrow in A.

This is the standard universal-algebra reading: A is the **free
algebra on five generators** modulo the Tambara composition law,
the functoriality law for shift, and the monad laws for settle
(over Imperfect). Closure means *the algebra is closed under its
own composition operation*, not *the generating set is closed*.

This is what makes the recognition load-bearing. A type system
whose generators are *known and finite* admits a structural analysis
that one whose generators are open-ended (Haskell's typeclass
hierarchy, Idris's interfaces, Lean's typeclasses) does not. Five
generators bound the dimensionality of the algebra; the substrate's
claim is that those five suffice for every type-formation rule the
language needs.

### 3.3 Type formation = fragment composition

The type of a fragment is its altitude plus its transparency at that
altitude. Constructing a fragment IS forming its type, in two steps
(per the lifecycle reading of §2.3):

1. **Address.** Fate emits au with `content: oid` (deferred lift to
   `content: SpectralUuid` per §9.7) and a declared altitude. This
   produces au's identity. Per the parametric-types insight, this is
   `settle(T)` at the type layer in its emission sense: au IS the
   proposed witness of having been at the altitude it claims.

2. **Verify.** Run au through the property chain (`reflect` per
   `boot/std/epistemologic/property.mirror`) at the altitude's
   property set. The verdict lifts into `Imperfect`:
   - **Pass** (residual transparency weighs zero) → fragment
     committed to `@mirror/store` at the declared altitude.
   - **Partial** (located opacities remain) → fragment committed
     with the opacity_map carried in its transparency; the kintsugi
     loop reads it to pick the next focus.
   - **Failure** → au stays in Fate's buffer; if no fill converges,
     the fragment surfaces as dark under `total_classification` (per
     `docs/specs/strict-and-total-classification.md`).

Type checking IS the kintsugi loop on the fragment manifold of a
project. **eⁿ⁺¹ ≤ eⁿ at the type system altitude** is the substrate-
level statement that the type checker terminates with strictly
decreasing residual on every step that takes a fill, OR with zero
residual at the fixed point. Per `[[architecture-connes-spectral-triple]]`,
this is the Rayleigh quotient of D acting on H — the substrate's
Dirac operator descending the fragment's transparency.

---

## 4. The Connes Spectral Triple Grounding — SpectralUuid as the Bridge

Per `[[architecture-connes-spectral-triple]]` and
`docs/specs/prism-core-as-spectral-triple.md`, the substrate IS the
operational form of Connes' (A, H, D). Seam's adversarial review
(2026-06-06, F4) flagged that the previous §4 identification of H
with `fragment(altitude)` did not declare an inner product,
completeness, or scalar/vector structure — the void document's
grounding (Braunstein/Ghosh/Severini 2006) gives a finite-dimensional
Hilbert space *per OID graph*, not a global manifold of fragments-as-
vectors. The honest construction has two layers, sharpened by Alex's
2026-06-06 three-layer recognition:

**H is the Hilbert space of shards, spanned by SpectralUuids;
splinters are the basis units; the spectral structure is the
SpectralUuid layout.** Per-graph H_G ranges over the splinter OID-
graph within a shard; the cross-graph bridge IS SpectralUuid.

### 4.1 Local H and global H — best of both worlds

**Local H (rigorous, finite-dimensional).** For each connected OID
graph G with vertex set V(G) and edge set E(G) — i.e., for each
shard's splinter OID-graph — the Laplacian `L_G = D_G - A_G` (degree
matrix minus adjacency) is a positive semidefinite operator on
`ℂ^{|V(G)|}`. Per Braunstein/Ghosh/Severini 2006, the normalized
Laplacian `ρ_G = L_G / tr(L_G)` is a density matrix; the associated
Hilbert space `H_G = ℂ^{|V(G)|}` carries the splinter basis within
that shard. Within a single shard's OID graph, every splinter is a
basis vector labeled by its oid (and the shard itself by its
SpectralUuid); the inner product is the standard Euclidean /
Hermitian one; the spectral decomposition of `L_G` gives the local
λ₀ = 0 ground state (the connected-component indicator) and the
higher Laplacian eigenvalues.

**Global H (operational, via spectral_uuid).** Across shards (each a
distinct OID graph), the manifold of shards is not a single Hilbert
space in the standard sense (the per-graph spaces have different
dimensions; the direct sum is too large; the limit is undefined).
The **substrate's operational construction** uses spectral_uuid as
the cross-shard bridge:

- `spectral_uuid` (declared at the substrate altitude in
  `shards/mirror/spectral_uuid.mirror` as `@mirror/store/spectral_uuid`,
  2026-06-06 Mara's tick; realized in `prism/core/src/spectral_uuid.rs`;
  per `[[architecture-shard-as-crdt]]`) is a 128-bit identifier with a
  golden-ratio split: **48 bits ACTIVE** (quantized
  `SpectralCoordinate<5>` — navigable; carries local Laplacian
  neighbourhood structure) + **80 bits DARK** (BLAKE3-truncated
  content hash — identity).
- The active portion is **graph-navigable**: it encodes the local
  spectral coordinate of the fragment within its home graph G.
  Per the design discipline of task #124, SpectralUuid was constructed
  collision-resistant + fast + **graph-navigable** + Fortran-
  implementable; graph-navigability IS the property that the local
  Laplacian neighbourhood is encoded in the active bits.
- `spectral_uuid` is a **monoid homomorphism** w.r.t. shard merge
  (per `docs/specs/reality-shard-as-crdt.md` §3, declared as
  `requires monoidal(spectral_uuid)` in
  `shards/mirror/spectral_uuid.mirror`):
  `spectral_uuid(merge(a, b)) = combine(spectral_uuid(a), spectral_uuid(b))`.
  This makes the algebra of addresses compose cleanly with the
  algebra of fragments.

**The bridge.** The Connes distance functional
`d(p, q) = sup{ |f(p) - f(q)| : f ∈ A, ‖[D, f]‖ ≤ 1 }` becomes
**operational** because the SpectralUuid difference between two
shards encodes the spectral state. Computing d(p, q) does not
require evaluating the sup over all bounded-commutator a ∈ A; the
local Laplacian neighbourhood structure on the active 48 bits IS
the local spectral data d(·,·) integrates. Per the design
discipline, this gives **substrate-native distance** between shards
without a global Hilbert-space construction.

### 4.2 The Connes triple identification, sharpened

| Component | Mosaic-as-type-system reading | Where it lives in the substrate |
|---|---|---|
| **A (algebra)** | `@mirror/mosaic` — the five operations as generators of the type-formation algebra acting on the shard manifold (composing splinters into settled shards) | `shards/mirror/mosaic.mirror`, `shards/prism.mirror`; Rust: `prism/core/src/bundle.rs`'s trait chain |
| **H (Hilbert space)** | **Local:** `H_G = ℂ^{\|V(G)\|}` per shard's splinter OID graph G, with splinters as basis vectors and the shard itself labelled by its spectral_uuid; inner product canonical; ρ_G = L_G / tr(L_G) the density matrix per Braunstein/Ghosh/Severini 2006. **Global:** the family `{H_G}_G` indexed by shards (each shard IS one OID graph), bridged by spectral_uuid's monoid homomorphism on shard addresses. The Hilbert space of shards is spanned by spectral_uuids; splinters are the basis units. | `shards/glass.mirror`'s `splinter(altitude)` (basis units) and `shard(altitude)` (the spanned vectors, with `id: spectral_uuid` per the 2026-06-06 lift); `shards/mirror/spectral_uuid.mirror` declaring `@mirror/store/spectral_uuid`; `prism/core/src/spectral_uuid.rs`; `docs/specs/reality-shard-as-crdt.md`; `[[reference-void-document]]` |
| **D (Dirac operator)** | The kintsugi flow — operationally **the splinter-composition-into-shard transition operator.** Each settle iteration shifts SpectralUuid; D's action on a shard IS that shift. D maps a shard's current SpectralUuid to the SpectralUuid the recomposed splinter set would carry under decreased residual transparency; D's spectrum IS the residual opacity profile across the kintsugi loop's iterations | `docs/specs/kintsugi-wiring.md`; `docs/specs/au-and-conductivity.md`; `prism/core/src/spectral_uuid.rs`'s `combine` (per §11 open question 1, the quantized arithmetic is still being pinned); `terni::Imperfect<State, _, Holonomy>` per `prism/imperfect/` |

### 4.3 What the SpectralUuid bridge collapses

Seven concrete recognitions follow from naming SpectralUuid as the
operational form of the spectral state:

1. **Content-addressing IS spectral state encoding.** One mechanism
   (the SpectralUuid layout), two roles (identity via dark bits,
   spectrum via active bits). Not two systems plumbed together.
   The shard.id IS both at once.

2. **Connes distance becomes computable.** The SpectralUuid
   difference between two shards IS the substrate-native distance
   metric. The standard sup-over-bounded-commutators construction
   reduces to a 48-bit arithmetic difference in the active portion,
   modulo the quantized composition rules of SpectralCoordinate<5>.

3. **D becomes operational as splinter-composition.** D is the
   splinter-composition-into-shard transition operator during
   kintsugi settle: each tick recomposes the splinter set; the
   active bits of the resulting shard's SpectralUuid update under
   the local Laplacian neighbourhood across the new composition;
   the dark bits update under the content hash of the new splinter
   set. The operator is implementable; it does not require a
   separate axiomatic construction.

4. **Eigenboard becomes literal.** The algedonic surface (per the
   eigenboard cellular-sheaf reading) reads SpectralUuids directly,
   not derived metrics. Cross-shard coordination reads off the active
   bits; per-shard identity reads off the dark bits.

5. **Refactoring has a substrate-native cost metric.** The SpectralUuid
   distance between an old shard and a candidate replacement IS the
   refactor cost. This is the operational form of "how big a change
   is this?" expressed in substrate vocabulary, not in line-count or
   diff-size metrics. Splinter-level changes propagate up to the
   shard's SpectralUuid through the monoid homomorphism.

6. **Cross-altitude distance becomes meaningful.** SpectralUuid
   carries spectral structure across altitudes — `@code/rust`,
   `@code/llvm`, `@code/fortran`, `@nl`, `@release`. The active bits
   are altitude-invariant up to the basis-transformation operator
   `shift`; the dark bits are altitude-specific (because the content
   bytes differ per altitude). Cross-altitude distance is the active
   difference *plus* a shift cost.

7. **§9.5's punt becomes a research roadmap.** Connes' axioms —
   orientability, Poincaré duality, first-order condition, reality
   structure — become specific axioms to verify *against the
   SpectralUuid-bridged construction*, not against a gestured
   construction. Each axiom translates to a property of
   SpectralUuid's composition rules. §9.5 below restates this.

### 4.4 What this changes from the previous §4

- The previous §4 named H as "the void document's quantum information
  manifold" without declaring inner product, completeness, or basis
  structure. The construction was structural-suggestive but not
  operational. Seam F4 was correct.
- The corrected §4 names two H's: **local** (rigorous, per Braunstein
  et al. 2006) and **global** (operational, via SpectralUuid).
- The previous §4 named D as "the kintsugi flow" gesturally. The
  corrected §4 names D as the SpectralUuid-update operator, with
  Connes' bounded-commutator condition `‖[D, a]‖ < ∞` realized via
  the bounded active-bit arithmetic.
- The Connes axioms now have a concrete verification target (the
  SpectralUuid composition rules), not an abstract one.

**Why this makes mirror's substrate honest about its mathematical
lineage.** Connes' (A, H, D) is the canonical noncommutative-geometry
framework. The literature on spectral triples is forty years deep and
covers gauge theory, the Standard Model of particle physics, quantum
gravity, and information geometry. Mirror does not invent the
framework; mirror applies the framework at the type-system altitude,
**with SpectralUuid as the operational bridge that makes the
application computable rather than gestural.** The contribution is
the application + the recognition that `@mirror/mosaic`'s five-
operation algebra IS the A of a finite spectral triple, with the
proof obligation eⁿ⁺¹ ≤ eⁿ realizing the spectral action's monotone
descent on residual transparency.

The void document closes this: λ₀ = 0 is the autopoietic closure of
the manifold of fragments (Lawvere fixed point, Soto-Andrade &
Varela 1984). The kintsugi loop descends to λ₀; the settled fragment
IS the fixed point. `SpectralUuid::EMPTY` (active = 0, dark =
BLAKE3-of-empty) IS the substrate's first named address at λ₀. The
proof obligation is operational; the math is canonical.

---

## 5. The Lineage — Standing on Shoulders of Giants

Per Alex's framing, `@mirror/mosaic` is "the 'we're standing on the
shoulders of giants' part of the language." Each substrate decision
inherits from a named tradition. The contribution is the **composition**,
not any single inheritance.

### 5.1 Lambda calculus (Church 1936)

Composition is reduction. Mirror's `|>` pipeline operator and the
five operations compose by left-to-right reduction; mirror's
substrate-pull `lift` (formerly `zoom`, per
`[[architecture-lift-as-load-bearing]]`) is the variety-preservation
move that names *which* reductions preserve the manifold's
information geometry.

**Where mirror best inherits.** The closure of composition. Every
mirror program is a finite composition of the five operations on
fragments; reduction always terminates (sub-Turing).

**Where mirror diverges.** Mirror does not have general recursion;
lambda calculus does (via the Y combinator). Mirror's totality
discipline (`@epistemologic/property/total_classification`) is
strictly stronger than untyped lambda calculus's termination
guarantee (which is absent).

### 5.2 Total functional programming (Turner 2004)

Per Turner's "Elementary Strong Functional Programming," every
function in a total functional language terminates. Idris, Agda, Coq,
Lean, Charity, and Dhall all carry the discipline. Mirror inherits
the totality contract directly — `settlement IS termination` per
`docs/specs/type-theory-position.md` §1.1.

**Where mirror best inherits.** The "all programs terminate" guarantee.
Mirror's sub-Turing claim is the same shape as Turner's strong
normalization. Per the Hacker News reference thread on Turner's paper:
"the big Idea is that the language is non-Turing complete and
termination is guaranteed." Mirror takes this verbatim and adds the
content-addressed substrate as the verification mechanism (the oid IS
the witness that the computation halted with a definite content).

**Where mirror diverges.** Turner's languages (Miranda, the
hypothetical total ML) use structural recursion checkers (foetus,
Agda's termination checker). Mirror uses the kintsugi loop's
monotone descent (eⁿ⁺¹ ≤ eⁿ) and the spectral triple's bounded D —
a different verification mechanism that lands on the same guarantee.

### 5.3 Dependent type theory (Martin-Löf 1972)

Per-Martin Löf's intuitionistic type theory: types depend on values.
A `Vect n A` is a vector of length `n`; building the value verifies
the index. Mirror's `au(altitude)` is structurally identical: the
altitude is a value-level ref, the au's type depends on it, and
constructing a `au(@code/rust)` value IS the proof that it settled
at that altitude.

**Where mirror best inherits.** The "type depends on value" idiom.
Every parametric type in mirror is a dependent type in the
Martin-Löf sense — `fragment(altitude)`, `au(altitude)`,
`transparency(property)`, `ast(grammar)` — and the value-level ref
is what makes them distinct types.

**Where mirror diverges.** Martin-Löf type theory is dependently
typed all the way down (any value can index any type). Mirror's
dependency is restricted to the altitude/property/grammar parameters
named in the substrate. The trade is: less polymorphism (you cannot
parameterize on arbitrary values), more inference (the altitude is
always known statically because the substrate names it).

### 5.4 Calculus of inductive constructions (Coquand & Huet 1985)

Per Coquand & Huet's CoC: types and terms in one universe; inductive
definitions land. Per `[[architecture-prism-as-trait-as-everything]]`:
mirror's `prism` IS trait IS type IS grammar. The four "different
things" collapse to one declaration form. This is the same move CoC
made for types and terms; mirror extends it one universe further to
traits and grammars.

**Where mirror best inherits.** The collapse of universes. CoC has
`Set`, `Prop`, and `Type_i` as different universes; mirror has
`prism` as the one declaration form that covers what other languages
split across multiple universes. The collapse is what makes the
algebra finitely generated (five generators per §3.2).

**Where mirror diverges.** CoC's universe hierarchy (predicative or
impredicative) is open-ended (`Type_0 : Type_1 : Type_2 : ...`).
Mirror's prism algebra is fixed at five operations. The dimensionality
is bounded by construction.

### 5.5 Category theory (Mac Lane 1971, Eilenberg & Mac Lane 1945)

Mosaic IS a category. Objects are fragments; morphisms are
applications of the five operations; composition is the Tambara law.
The functor laws hold for `shift` (per
`[[architecture-operations-as-linear-algebra]]`); the monad laws
hold for `settle`; the limit/colimit constructions are `focus`,
`project`, `split` (per §3.1 above with the caveats).

**Where mirror best inherits.** Categorical composition. The Tambara
module composition law (per Clarke et al. 2020,
`docs/specs/type-theory-position.md` §2.7) realizes mirror's
composition table as the categorical update on profunctor optics.

**Where mirror diverges.** Mirror does not need the full categorical
machinery. The seven AST variants in
`docs/specs/type-theory-position.md` §1.3 are the seven specific
optics mirror's algebra realizes; the full profunctor optics lattice
(Iso, Lens, Prism, AffineTraversal, Traversal, Fold) maps onto them
but mirror does not declare the lattice as a separate framework. The
algebra closes at five operations, not at the full optics lattice.

### 5.6 Curry-Howard correspondence (Curry 1934, Howard 1969)

Proof-as-program. A program of type T is a proof of proposition T.
Mirror's transparency carries the proof: a fragment with
`transparency: success` at altitude A IS a proof that the fragment
settles at A. `verify(au) -> verdict` is the operational form of
the proof-checking judgement.

**Where mirror best inherits.** Proof-carrying constructions.
Building a fragment IS proving its conductivity at the altitude
(per the parametric-types insight: `settle(T)` is the verified-
construction form). The proof is the value's existence + its
transparency.

**Where mirror diverges.** Curry-Howard typically realizes the
correspondence in constructive logic (intuitionistic, no excluded
middle). Mirror's verdicts are three-valued
(`pass | partial(confidence) | failure(reason)`), which is closer
to many-valued logic than to classical or intuitionistic logic.
The continuous loss carrier (`transparency<p>.weight: f64`) is a
quantitative refinement of the Curry-Howard idiom: not just "is
there a proof?" but "how close is the proof to closing?"

### 5.7 Connes noncommutative geometry (Connes 1985-1996) — operational with named axiomatic gaps

The spectral triple (A, H, D). Mirror's substrate IS the operational
form per §4 above: A = mosaic (the five-operation algebra acting on
shards by composing/decomposing their splinter sets); H = the
Hilbert space of shards (the {H_G} family bridged by SpectralUuid,
with per-graph H_G grounded in Braunstein/Ghosh/Severini 2006 and
spanned by SpectralUuids; splinters are the basis units); D = the
splinter-composition-into-shard transition operator during kintsugi
settle. The literature: Connes' 1994 "Noncommutative Geometry"
textbook; Connes 1996 "Gravity coupled with matter and the
foundation of non-commutative geometry" (the reconstruction
theorem); Connes & Chamseddine 1997 "The Spectral Action Principle";
van Suijlekom 2014 "Noncommutative Geometry and Particle Physics"
(Standard Model from finite spectral triples).

**Where mirror best inherits.** The (A, H, D) framework as a unified
operational shape for a type system, **bridged operationally by
SpectralUuid** rather than left structural-suggestive. Per
`docs/specs/type-theory-position.md` §5.5: "the spectral triple
interpretation is suggestive and the mathematical structures align."
This spec sharpens "suggestive" to "operational with named axiomatic
gaps" — the gaps are the unverified Connes axioms (orientability,
Poincaré duality, first-order condition, reality structure), each of
which becomes a specific verification target against the SpectralUuid
construction (see below).

**Where mirror diverges.** Connes' axioms for spectral triples have
not been formally verified for mirror's (A, H, D). This spec does NOT
claim verification. It claims the verification target is now concrete
rather than gestural:

| Connes axiom | What it would mean for mirror | Verification target |
|---|---|---|
| **Bounded-commutator** ‖[D, a]‖ < ∞ for all a ∈ A | Each mosaic operation, applied to a fragment, produces bounded change in SpectralUuid distance | Show that for each of the five generators, the active-bit difference between input and output SpectralUuids is bounded by a constant depending only on the generator. The bound exists per the quantized 48-bit arithmetic; the explicit constant is research work. |
| **Orientability** | The fragment manifold carries a consistent orientation (a Hochschild cycle of dimension n with `π_D(c) = γ` the chirality operator) | Identify the chirality operator in mirror's substrate. Candidate: the in/out polarity of `project` (declared in `shards/prism.mirror:68-69` as `project in(prism) / project out(prism)`). Verification: show the in/out distinction realises the ±1 eigenspace of a γ on H_G. |
| **Poincaré duality** | The fragment manifold's K-theory is dual to its K-homology | Identify the K-theory class of `@mirror/store`. Candidate: the OID graph's first Betti number (cycle structure) corresponds to K_0; the zeroth Betti number (connected components) corresponds to K_1. Verification: deep mathematical work. |
| **First-order condition** | [[D, a], b] = 0 for all a, b ∈ A acting from the right | Two non-overlapping mosaic operations commute up to opacity. Verification: show that the active-bit arithmetic respects this when the operations act on disjoint fragment regions. |
| **Reality structure** | A real structure J on H satisfying JD = εDJ for some sign ε | Identify J in mirror's substrate. Candidate: the `imperfect` functor's `success`/`failure` symmetry, or the conjugation arising from `@mirror/store`'s symmetric content-addressing. |

The closest verified target today: Connes' bounded-commutator
condition `‖[D, a]‖ < ∞ for all a ∈ A`. Per
`docs/specs/prism-core-as-spectral-triple.md` §"Why
`Imperfect<State, _, Holonomy>` is the Dirac signature": mirror's
`Transport::transport`'s `Imperfect` return type encodes the
bounded-residual condition. Combined with SpectralUuid's bounded
48-bit arithmetic, the boundedness side of the axiom is
structurally implied; the explicit constant per generator is the
remaining research step.

**The honest qualification.** Mirror has not been formally shown to
satisfy all the axioms of a noncommutative geometry. The axioms now
have concrete verification targets against the SpectralUuid-bridged
construction, each translatable to a property of SpectralUuid's
composition rules or the substrate's existing polarities (in/out,
success/failure). This is the difference between "structural
suggestive" (the previous claim) and "operational with named
axiomatic gaps" (the upserted claim). Closing each gap is a research
arc, not a single tick. §9.5 below restates this as the research
roadmap.

### 5.8 Content-addressed languages (Unison, IPLD/IPFS, Nix CA derivations)

Fragments are addressed by hash. Mirror inherits the discipline.

- **Unison** (per the Unison docs: "Each Unison definition is
  identified by a hash of its syntax tree. Put another way, Unison
  code is content-addressed.") hashes **both term definitions AND
  type declarations** — every definition in Unison's codebase
  (terms, types, abilities) is content-addressed by its hash. The
  previous draft of this spec mistakenly claimed Unison hashes only
  function definitions (Seam F5, correct). The honest comparison is
  **categorical, not extensional**: Unison's content-addressing is a
  **definition-identity scheme** (the hash IS the canonical name of
  the definition; names are aliases for hashes). Mirror's content-
  addressing is a **typing-time parameter** (`fragment(altitude)`
  where altitude TYPES the hash; two fragments with byte-equal
  content at different altitudes ARE different fragments). Different
  category. Unison's hash collapses names into identity; mirror's
  hash carries the altitude as part of the type and uses content-
  addressing as a substrate property the type system reads.

  Where mirror's reach extends past Unison's: mirror's content-
  addressing applies to **every altitude** (AST nodes, build targets,
  deployment shards, peer identities, spec targets, transparency
  carriers), not only to definitions. Unison's hashing is restricted
  to the language's own definitions; mirror's is a substrate property
  visible at every altitude where the substrate names things.

- **IPLD** (InterPlanetary Linked Data, the data model under IPFS)
  provides content-addressed data structures with CID (content
  identifier) as the universal address. Mirror's oid is structurally
  identical; the substrate difference is that mirror's oid is typed
  at its altitude, where IPLD's CID is untyped (or weakly typed via
  the codec).

- **Nix CA derivations** (per the NixOS Discourse: "content-addressing
  derivations are the right technical basis for doing that correctly")
  content-address build outputs. Mirror's `au(altitude)` is the same
  shape — the build output's identity IS the hash of its content + its
  altitude. Mirror inherits the structural discipline; the difference
  is that mirror's altitude is a first-class type parameter, where
  Nix's derivation altitude is implicit in the derivation's name.

**Where mirror best inherits.** Content-addressing as universal
identity. The discipline that "two things with byte-equal content ARE
the same thing" makes equality decidable and makes the Merkle property
(verify a leaf, verify the tree) automatic.

**Where mirror diverges.** Mirror lifts content-addressing into the
type system. Unison's hashes name function identities; mirror's oids
ARE the typing witnesses. Per the parametric-types insight: building
a `crystal(t)` value IS the proof of having computed something of
type `t` whose hash matches the bound oid. No other content-addressed
language (Unison, IPLD, Nix) has lifted addressing to a typing
primitive in this way.

### 5.9 Linear/affine types and quantitative type theory (Wadler 1990, Atkey 2018)

Linear types (Wadler) require each value to be used exactly once.
Affine types allow zero or one use. Quantitative Type Theory (Atkey
2018, Orchard et al. 2019) generalizes to a semiring of usage grades.

Mirror's `lift` (variety-preservation, per
`[[architecture-lift-as-load-bearing]]`) is the same axis. Lift is
the prism operation that preserves the manifold's variety under
composition; the variety-preservation property
(`@epistemologic/property/variety_hold`) is the substrate's
linear/affine analog.

**Where mirror best inherits.** Variety preservation as a structural
property. The kintsugi-variety spec (`docs/specs/kintsugi-variety.md`)
declares variety_hold as a load-bearing property; the loss carrier
(`transparency<p>.weight`) is a semiring grade in the QTT sense.
Per `docs/specs/type-theory-position.md` §1.2: "Mirror's loss monoid
is a grading structure. ... mirror's type system [is] a form of
**quantitative type theory** (QTT), where the semiring of grades
tracks information loss rather than resource usage."

**Where mirror diverges.** QTT's semiring is abstract; mirror's
semiring is always information-theoretic (Shannon-like, per the
transparency spec's open question §10.1). The discipline is
specialized to a specific physical interpretation (bits of
missing information).

### 5.10 Propagator networks (Sussman & Radul 2009)

Per Sussman & Radul's "The Art of the Propagator": "autonomous
machines interconnected by shared cells through which they
communicate." A propagator network settles by monotone convergence
of partial information at each cell.

Mirror's kintsugi loop is structurally a propagator network at the
substrate altitude. Each fragment is a cell; each operation is a
propagator; transparency monotonically descends as fragments settle.
Per `docs/specs/au-and-conductivity.md`'s "stage 3 conductivity
contest": Fate's tournament IS a propagator selecting from candidate
fillings; the winning fill is the one whose conductivity is clearest
in this context.

**Where mirror best inherits.** Monotone descent on partial
information. A propagator's commitment-only-when-information-arrives
discipline matches mirror's `transparency: partial(opacity_map)`
shape — partial fragments are honest about what they don't know yet.

**Where mirror diverges.** Sussman & Radul's propagators are
general-purpose constraint solvers; mirror's kintsugi loop is
specialized to the (A, H, D) spectral triple's descent. The
algorithm is the same shape; the geometric grounding is mirror-specific.

### 5.11 What mirror does NOT cleanly inherit

Be honest about the strains:

- **Linear logic (Girard 1987).** Mirror has no exponential modality
  (`!A`), no resource discipline at the operation level. The closest
  analog is variety_hold, which is structurally different
  (information-theoretic, not resource-bounded).

- **Cubical type theory (Cohen et al. 2018).** Mirror has no
  higher-inductive types and no path types. The closest analog is
  the kintsugi loop's monotone descent, which is a 1-categorical
  thing; mirror does not lift to ∞-categorical reasoning.

- **Effect systems (Koka, Eff).** Mirror has `@io` as the effect
  boundary, but does not track effects in row-polymorphic fashion.
  Per `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md`
  §7: "Effects are platform surfaces, not type-system distinctions."

- **Refinement types (Liquid Haskell, F\*).** Mirror's verdicts are
  closer to many-valued logic than to SMT-checked refinements; per
  `docs/specs/type-theory-position.md` §2.6, the verdicts are
  "continuous all the way down" where refinements are discrete.

Naming these honestly is part of the substrate-pull discipline:
mirror's lineage is real where it claims to be, and mirror's
deviations from the literature are intentional (sub-Turing-by-design,
five-op-closure, content-addressing-as-type).

---

## 6. What Collapses on the Rust Side

The recognition has operational consequences. Per
`butterfly-self-hosting.md` §5 the cut criteria are measurable;
this section names the LOC-level shrinkage the type-system
recognition unlocks.

**Currently** (active mirror bootstrap, per `bootstrap/src/`):

- `cmd_kintsugi_spec` (in `bootstrap/src/main.rs`) — dispatches when
  the input is a `mirror.spec` file; text-walks the spec via
  `parse_spec_targets`; produces a list of `SpecTarget`s; runs
  kintsugi on each.
- `cmd_kintsugi_ci_single` — dispatches on a single `.mirror` file;
  emits the `--ci` verdict envelope.
- `cmd_kintsugi_ci_corpus` — dispatches on a directory tree; walks
  the corpus; aggregates verdicts.
- `parse_spec_targets` (in `bootstrap/src/main.rs`) — the text-walker
  that reads `target NAME { ... }` blocks out of a `mirror.spec` file.
  Hand-written; brittle against the spec grammar drifting.
- `SpecTarget` (struct in `bootstrap/src/main.rs`) — a separate shape
  carrying `name`, `altitude`, `emit`, `cli`, `needs`.

**Under the recognition** (post-this-spec, post-fragment-floor):

- One `cmd_kintsugi` dispatch that takes a fragment as input. The
  fragment's altitude tells the dispatcher which kintsugi pass to
  run. A `mirror.spec` file at altitude `@mirror/spec` runs the spec
  kintsugi; a `.mirror` file at altitude `@meta/ast` runs the AST
  kintsugi; a directory tree at altitude `@corpus` runs the corpus
  kintsugi. Same dispatch, same loop, three altitude-specific reads.

- `parse_spec_targets` retires. The substrate-level reading of a
  `mirror.spec` file IS evaluating its content under `@mirror/spec`
  (per `shards/mirror/spec.mirror`). The spec-altitude grammar parses
  its own input as fragments; the text-walker becomes the grammar's
  internal mechanism, not a Rust-side hand-walker.

- `SpecTarget` retires. A target is a fractal-shape fragment whose
  content includes the target's name + altitude + emit. The Rust
  side reads it via the same `Fractal<E, H>` API every other
  fragment uses.

**LOC estimate.** Per `bootstrap/src/main.rs` (currently ~69.8KB,
roughly 1800 LOC), the kintsugi-related commands and
`parse_spec_targets` account for roughly 400-600 LOC. The
fragment-shaped dispatch is ~100 LOC. Net shrinkage: 300-500 LOC,
plus retirement of `SpecTarget` (~50 LOC) and `parse_spec_targets`
(~100 LOC). **Substrate-pull-realize:** the Rust shrinks because
the substrate names what the algebra IS, and the per-altitude
special-casing was a workaround for not having that name.

This is the same shrinkage pattern named in
`docs/specs/prism-core-as-spectral-triple.md` §"What retires from
the bootstrap" — total bootstrap from ~5000 LOC to ~1200 LOC, with
the difference moving to grammar declarations. The type-system
recognition makes one of the larger retirement passes concrete: the
kintsugi-dispatch layer collapses around fragment as the dispatch
shape.

---

## 7. What This Opens

### 7.1 The butterfly path

Per `roadmap/wip/butterfly-self-hosting.md` (landed 2026-06-06 at
`dcb9dc4`): the butterfly is the small complete self-hosted mirror
binary whose codegen contract is closed under the substrate's own
algebra. The butterfly's runtime IS fragment-shaped.

**The recognition makes this load-bearing.** Per
`butterfly-self-hosting.md` §6: "Once the butterfly flies:
HamiltonScheduler runs in the butterfly's own runtime, per shard.
... SpectralSupervisor sits on top, as a separate `au(@code/llvm)`
artifact." The "shards" running under HamiltonScheduler are
fragments at the runtime altitude. The "au artifacts" are settled
fragments at the codegen altitude. The four transports (λsh /
mirror-CLI / MCP / LSP) are glasses on `prism @mirror/cli` per
`cli-as-prism.md`; each is a fragment at the transport altitude.

**Spaceland becomes literal.** Per `docs/specs/the-convergence.md`
and `docs/specs/cli-as-prism.md`: one runtime, four transports, five
operations. Each transport is a window onto the same fragment
manifold; each operation acts identically across transports because
the underlying fragments are the same. The metaphor flies clean
because the substrate-pull does — every altitude is a window onto
the same composition.

### 7.2 MCP/LSP/CLI/λsh project from one fragment graph

Per the convergence specs: the four transports share the runtime.
Under this spec's recognition, they share the type system. A fragment
at altitude `@meta/ast` is the same object whether read through MCP's
tool-call interface, LSP's textDocument interface, the CLI's
`mirror compile` interface, or λsh's `mq` query language. The four
transports differ in their wire protocol (per `the-convergence.md`)
but compose against the same fragment-graph.

This unlocks the missing piece of `docs/specs/lsp-and-mcp.md`'s
shared abstraction: the LSP `textDocument/diagnostics` notification
and the MCP `tools/call` response are both fragments at altitude
`@diagnostic` and `@tool_result` respectively, settled by the same
kintsugi pass that emits the bytes. The Rust side does not need a
separate `LspNotification` or `McpToolResult` type — both reduce to
`Fragment<altitude>`.

### 7.3 The void document made operational

Per `[[reference-void-document]]`, every connected graph lives on a
path between Splinter (K_n, maximum entanglement) and Narcissus
(K_{1,n-1}, minimum entanglement). The manifold of fragments IS that
graph — fragments connected by their OID references, with
content-disjoint splits at Splinter (a complete graph of unique
fragments) and content-shared concentrations at Narcissus (a hub
fragment that many others reference).

The kintsugi loop's monotone descent IS the discrete Ricci flow
between these poles. λ₀ = 0 is the generative zero where all eight
dualities meet — the settled fragment whose transparency is success
and whose oid is the autopoietic closure of its own composition.

This was geometric framing without an operational interface. The
recognition makes it operational: the substrate's type system IS the
manifold of fragments + the kintsugi descent. Programs settle at λ₀.
Types check by descending to λ₀. The void document's eight dualities
are observable at every typing judgement.

---

## 8. Migration Path

Two ticks. Substrate-pull. Earn their keep.

### Tick 1 — Land the splinter/shard floor (this spec + glass.mirror addition)

**Status: this spec is the tick.** The glass.mirror addition (the
`splinter(altitude)` atom + `splinter_shape` disjunction + the
`shard(altitude)` settlement type) ships alongside this spec on
`main`. The Rust side does not change yet — `Fractal<E, H>` continues
to be the realization; the substrate declaration names the algebra;
nothing breaks. Cut criterion: glass.mirror parses; the spec lands;
the `[[architecture-glass-wall-substrate-types]]` entry gains a
forward reference to this spec.

This tick is **substrate-pull-realize**: a naming move that closes a
loop. It does not gate v0.1.0; the splinter/shard floor is
post-v0.1.0 architectural recognition (per the brief). It does open
the next tick.

### Tick 2 — Collapse the kintsugi dispatch around splinter / shard

The Rust-side work named in §6. One `cmd_kintsugi` dispatch on
splinter (input form) and shard (output form);
`cmd_kintsugi_spec` / `cmd_kintsugi_ci_single` /
`cmd_kintsugi_ci_corpus` collapse into one. `parse_spec_targets`
retires; `SpecTarget` retires. The kintsugi pass reads the splinter's
altitude and dispatches the altitude-specific kintsugi step that
settles into a shard.

This tick is **substrate-pull-realize at the Rust layer.** It is not
required for v0.1.0; it is the consequence of Tick 1 that the
butterfly will need to fly clean. Sequence: land after v0.1.0 cut;
land before Phase 4 emit-self work begins (because Phase 4 will write
the kintsugi pass as a `.mirror` grammar, and the grammar's surface
should be splinter/shard-shaped from the start).

Per `[[feedback-no-time-estimates]]`: one tick after the other. No
"~N weeks." The sequence is what matters; the timing is whatever it
is.

---

## 9. Open Questions

The genuinely unresolved ones. Reed will call them with Alex.

### 9.1 Does `splinter(altitude)` or `shard(altitude)` need a sub-prism declaration?

`shards/mirror/store.mirror` declares `@mirror/store/oid` as a sub-prism
(per the `glass @mirror/store/oid { ... }` block). Do `splinter(altitude)`
or `shard(altitude)` similarly want to be sub-prisms `@glass/splinter`
/ `@glass/shard` so their five-operation surfaces are callable directly?
Or are the type declarations at the glass-altitude sufficient, with
the five operations inherited from `@glass` itself? **Lean:** the type
declarations are sufficient; splinter and shard are *types* at the
glass altitude, not *prism* peers of glass. Sub-prism declarations
would be redundant.

### 9.2 What is the relationship between `splinter` (atom) and `splinter_graph` (OID-graph)? — Resolved

Resolved 2026-06-06 (second upsert). `shards/mirror/store.mirror`
previously declared `splinter` as the OID-graph
(`type splinter = { root: oid, children: [oid] }`). Under Alex's
three-layer recognition, this naming was reaching for two distinct
concepts: the atomic content-addressed unit (`splinter` at @glass)
and the dependency-closure projection (`splinter_graph` at @store).
The 2026-06-06 substrate edits split them explicitly:

- **`splinter`** at `@glass` — the universal atom: one
  (content, altitude, transparency) triple per atom.
- **`splinter_graph`** at `@mirror/store` — the OID-graph projection:
  root + transitive closure of children's OIDs; the structural
  lockfile.

A fractal-shape splinter carries `fractal([oid])`; a splinter_graph
is the flattened (root, children) view of a composition's closure
at @store's wire surface. They are not the same type: the atom is
the unit; the graph is a projection of a composition of atoms.

### 9.3 Does this spec retire `docs/specs/type-theory-position.md`?

The type-theory position spec is a research survey from 2026-05-19;
this spec consolidates its findings into a load-bearing recognition.
Should the type-theory position spec be archived (moved to
`docs/specs/historical/`) or kept as the citation pool this spec
draws from? **Lean:** keep both. The type-theory position is the
literature survey; this spec is the substrate-pull-realize step.
They serve different purposes.

### 9.4 At what altitude does `total_classification` apply?

Per `docs/specs/strict-and-total-classification.md`,
`total_classification` is the property that says every byte falls
into a recognized AST node. Under this spec's recognition,
`total_classification` is the property that says **every byte of a
fragment is content-addressed**, with no `Dark` shape representing
unclassified content. The two readings are compatible (the AST-altitude
reading is the special case of the fragment-altitude one). Does the
property need to be re-declared at the fragment altitude, or does
the existing AST-altitude declaration suffice? **Lean:** re-declare
at the fragment altitude when the AST is read as a fragment manifold.
This is Phase 5 work.

### 9.5 The Connes axioms — the verification roadmap

Reframed (2026-06-06 upsert) from "the formal status of the categorical
analogies" to **the operational verification roadmap for the Connes
axioms** per §4.3 entailment 7 and §5.7.

The substrate-primary characterisations (λ₀ / projection / decomposition
/ basis transformation / monad-close, per §3.1) are now load-bearing
without the categorical analogies needing to carry weight. The honest
question is whether Connes' four spectral-triple axioms hold for the
SpectralUuid-bridged construction:

| Axiom | Verification target (per §5.7) | Status |
|---|---|---|
| Bounded-commutator | Active-bit difference bounded by per-generator constant | Structurally implied; explicit constant per generator is research |
| Orientability | In/out polarity of `project` realizes ±1 chirality on H_G | Candidate identified; verification is research |
| Poincaré duality | OID graph Betti numbers ↔ K-theory / K-homology classes | Candidate identified; verification is substantial mathematical work |
| First-order condition | Active-bit arithmetic respects [[D,a],b]=0 on disjoint regions | Plausible; verification is research |
| Reality structure | `imperfect` success/failure symmetry as J operator | Candidate identified; verification is research |

**Lean:** treat each axiom as its own research tick, schedulable
independently. None are gating for v0.1.0. The substrate-pull-honest
position is: "each axiom has a concrete verification target; closing
them is a research arc; this spec does NOT claim verification, it
claims the targets are concrete."

The categorical analogies in §3.1 (where they hold — shift as functor,
settle as Kleisli arrow in Imperfect, split as coproduct-or-pushout)
are verified at the substrate-shard altitude: the laws are declared
in `@epistemologic/property/laws/{functor,monad}` and witnessed by the
substrate's content-addressing. They are not the load-bearing claims;
the spectral characterisations are.

### 9.6 Does the void-document λ₀ identification match the kintsugi
fixed point exactly?

Per §4 above and §7.3: the kintsugi loop descends to λ₀; the void
document names λ₀ as the autopoietic Lawvere closure. Per the upsert,
`SpectralUuid::EMPTY` (active = 0, dark = BLAKE3-of-empty) IS the
substrate's first named address at λ₀. Are the kintsugi fixed point,
the void document's autopoietic Lawvere closure, and SpectralUuid::EMPTY
the same λ₀, or are they three different fixed points that happen to
agree at the substrate altitude? **Lean:** they are the same. Per
`docs/specs/au-and-conductivity.md`'s "Formal statement: the tensor
is cycle-averaged holonomy" section and Soto-Andrade & Varela 1984.
The SpectralUuid layer makes the question concrete: the kintsugi
fixed-point check becomes "does the iterated settle converge to
SpectralUuid::EMPTY's active portion = 0?" — testable.

### 9.7 Should `fragment.content: oid` lift to `content: SpectralUuid`? — Resolved at the shard layer

Upserted 2026-06-06 (first); resolved 2026-06-06 (second upsert)
under Alex's three-layer recognition. The question framed in the
first upsert asked whether to lift `fragment.content` from `oid` to
`SpectralUuid`. Under the three-layer recognition that lift no
longer makes sense at the splinter (atom) layer:

- **Splinter.content stays as oid.** A splinter is the universal
  content-addressed atom; its identity IS its content hash; that's
  an oid by construction. SpectralUuid as an atom identifier would
  be a category error — the active 48 bits encode local Laplacian
  neighbourhood structure, which is a property of a shard's OID
  graph, not of an isolated atom.

- **Shard.id IS SpectralUuid by construction.** A shard is the
  SpectralUuid-addressed settled composition of splinters; the
  SpectralUuid IS its identity. The lift moves from "lift splinter's
  oid to SpectralUuid" to "declare SpectralUuid at the substrate so
  shard.id's slot tightens from oid to SpectralUuid." The shard
  declaration in `shards/glass.mirror` (2026-06-06) types
  `id: oid` as a placeholder against the deferred substrate-altitude
  SpectralUuid declaration.

**Why this lift is still deferred:**

- SpectralUuid is currently only a Rust type. Tightening `shard.id`
  to SpectralUuid requires first declaring SpectralUuid at the shard
  altitude (likely in a new `shards/mirror/spectral_uuid.mirror` or
  under `@mirror/store`).
- Per `docs/specs/reality-shard-as-crdt.md` §10, the SpectralUuid
  declaration is already queued behind T3 of fragmentation-mcp, with
  the quantized 48-bit `combine` arithmetic still being pinned (the
  spec's §11 open question 1).
- Doing the lift before SpectralUuid lands at the substrate would
  invent a primitive the rest of the substrate cannot construct.
  Per `[[feedback-no-bare-types]]`: always newtype — but newtype
  *into something declared,* not into something the substrate does
  not yet name.

**The trigger (closed 2026-06-06).** `shards/mirror/spectral_uuid.mirror`
landed Mara's substrate-pull-realize tick. The following substrate
edits landed in the same tick:

```mirror
# shards/glass.mirror
type shard(altitude) = {
  id: spectral_uuid,                # was: oid
  splinters: [splinter(altitude)],
  transparency: transparency(altitude),
}

# shards/mirror/store.mirror
type shard_ref = spectral_uuid      # was: oid
```

`spectral_uuid` is declared as a sub-prism at
`@mirror/store/spectral_uuid` parallel to `@mirror/store/oid` (the
two content-addressed identity carriers at the store altitude).
au's `content: oid` stays as it is — au is pre-commitment; the
spectral_uuid is what settle produces by composing the splinters
through the property chain. The property chain at commit time
continues to enforce spectral_uuid validity by construction.
**Closed:** the lift was mechanical; tick landed; the three-layer
recognition closes at the type altitude.

### 9.8 Where does property chain composition live, formally?

Upserted 2026-06-06. §2.3 names the property chain as the operational
form of `verify` and the substrate of settle's Kleisli arrow. The
chain composes through `Imperfect` (Pass-Partial-Fail propagation).
Is there an existing substrate spec that names how properties
compose via Imperfect, or is this an unwritten contract?

Current state:

- `boot/std/epistemologic/property/laws/monad.mirror` declares the
  monad laws for settle(T); it names the laws but does not enumerate
  the property chain's composition operator.
- `boot/std/epistemologic/property.mirror` declares `reflect(ast) ->
  [verdict]` and `check(ast, property) -> verdict`; this is the
  per-property surface, not the chain composition.
- `shards/glass.mirror:62-66` declares `imperfect(a, e, l)` with
  three variants; the composition is per the monad-laws shard.
- `docs/specs/au-and-conductivity.md` names conductivity as the
  context-dependent predicate; the chain composition rule is
  implicit (each property's verdict lifts into Imperfect; the chain
  is the Kleisli composition).

**Candidate resolutions:**

1. **Declare property chain composition as a substrate operation in
   `@epistemologic/property`** with explicit signature
   `chain([check]) -> check`. Lean toward this — it names the
   structure the substrate already uses implicitly.
2. **Treat chain composition as Kleisli composition of `imperfect`
   monad arrows**, inherited from `@epistemologic/property/laws/monad`.
   Lean second — it's the right abstract framing but doesn't surface
   the operational mechanism.
3. **Status quo:** leave it implicit. Push back.

**Substrate-pull-correct resolution lean:** option 1. Add a
`chain([check]) -> check` action to `shards/epistemologic/property.mirror`
(or `boot/std/epistemologic/property.mirror`) that lifts a list of
checks into a single check whose verdict is the Imperfect-monad fold
of its constituents. This is small. It would unblock subsequent
settle-as-property-chain claims being type-checkable. Capture as a
deferred substrate tick — not in this upsert. Trigger: when settle's
first altitude-specific property chain is implemented in a `.mirror`
shard.

### 9.9 Open questions from the 2026-06-06 second upsert (splinter/shard recognition)

Ambiguities surfaced during the three-layer recognition rename.
Captured here for triage; none gate v0.1.0.

1. **How does shard's SpectralUuid compose from splinter SpectralUuids?**
   The monoid homomorphism per `combine` in
   `prism/core/src/spectral_uuid.rs` is structurally implied (per
   §3 of `reality-shard-as-crdt.md`). The quantized 48-bit
   arithmetic for the active portion is still being pinned — this
   is `reality-shard-as-crdt.md` §11 open question 1. **Lean:**
   defer until SpectralCoordinate<5>'s eigenvalue composition rules
   quantize cleanly; the substrate naming is now precise enough
   that the arithmetic step is purely mathematical.

2. **Should `ShardRef` in `fragmentation/src/shard_ref.rs` rename?**
   No — `ShardRef` is already substrate-pull-honest. Its `id:
   SpectralUuid` and `context: ShardContext` shape match the new
   substrate-altitude `shard` declaration. The rename target is
   the splinter-shape variant: `Fractal::Shard → Fractal::Atom`
   (the leaf variant pre-dates the three-layer recognition that
   shard is the MIDDLE layer). **Lean:** defer the variant rename;
   add a `[substrate-pull:realize]` annotation on the next Rust
   touch of fragmentation/src/fragment.rs naming the deferred work.

3. **How does `splinter_graph` (the lockfile shape) compose into a
   shard's SpectralUuid?** The splinter_graph at @store is the
   OID-graph projection of a composition's closure; the shard's
   SpectralUuid is computed from the composed splinter set's
   monoid-homomorphism reduction. The relationship: the
   splinter_graph IS the @store wire surface for what gets fed into
   the monoid composition; the SpectralUuid is the result. They are
   not redundant; one is the on-the-wire form, the other is the
   computed identity. **Lean:** name this explicitly when the
   monoid composition lands in Rust.

4. **What does this mean for `cmd_kintsugi_*` Rust collapse?** §6
   names the collapse around fragment-shape; under the three-layer
   recognition, the collapse is around splinter (input) and shard
   (output). The dispatch reads the splinter's altitude; settle
   composes the splinter set into a shard; kintsugi iterates. The
   shape is now richer named: input/output types are distinct, not
   the same type at different lifecycle positions. **Lean:** the
   collapse aligns even better — the Rust code can name the input
   and output types separately instead of overloading one.

5. **What's the right home for SpectralUuid at the substrate
   altitude? — Resolved 2026-06-06.** Mara's tick landed
   `shards/mirror/spectral_uuid.mirror` declaring
   `@mirror/store/spectral_uuid` as a sub-prism parallel to
   `@mirror/store/oid` (the @mirror/store/oid precedent at
   `shards/mirror/store.mirror:78-94`). spectral_uuid surfaces as a
   typed reference (carrier: `ref`, following the no-bare-types
   discipline and the oid precedent) with three projection actions
   (`active`, `dark`, `combine`), the `fixed empty` bottom, and two
   property declarations (`content_addressed`, `monoidal`). The
   shard altitude (@glass) imports it; shard.id and shard_ref
   tighten in the same tick. The quantized 48-bit `combine`
   arithmetic stays deferred per `reality-shard-as-crdt.md §11
   OQ1` (the spec's open question). **Status:** §9.7 closed; the
   three-layer recognition is now load-bearing at the type altitude
   as well as at the algebra altitude.

---

## 10. Citations and Prior Art

### 10.1 Reed's practice insights

- `[[architecture-connes-spectral-triple]]` — the substrate IS the
  operational form of Connes' (A, H, D). Closes the deepest framing.
- `[[architecture-prism-as-trait-as-everything]]` — prism IS trait IS
  type IS grammar. The universe collapse.
- `[[architecture-operations-as-linear-algebra]]` — focus = λ₀
  eigenvalue; project = orthogonal projection; split = orthogonal
  decomposition; shift = basis transformation; settle = monad-close.
- `[[architecture-shards-as-substrate-source]]` — `shards/` is the
  destination; `boot/std/` is the legacy fallback.
- `[[architecture-glass-wall-substrate-types]]` — Imperfect +
  Transparency as substrate vocabulary.
- `[[architecture-fragmentation-is-the-rust-substrate]]` — the
  fragmentation crate is the Rust floor.
- `[[architecture-shard-as-crdt]]` — SpectralUuid as monoid
  homomorphism; the Connes-distance bridge identified in §4 of this
  spec. The substrate-pull-honest sense in which kintsugi composes
  with the CRDT layer.
- `[[architecture-pq-as-mcp-surface]]` and `[[architecture-shard-ref-as-prism]]`
  — ShardRef as the typed session handle (Prism + monoid); refract
  as lattice join. The runtime form of the spec/store seam this
  spec's §2.3 lifecycle reading rests on.
- `[[architecture-lift-as-load-bearing]]` — lift = variety
  preservation; the substrate-pull rename closing the loop with
  `boot/std/{option,result}.mirror`.
- `[[architecture-au-conductivity]]` — au is Fate's output type; gold
  conducts; verification IS conductivity in context.
- `[[reference-void-document]]` — eight dualities; Splinter (K_n) and
  Narcissus (K_{1,n-1}) as the poles of the quantum information
  manifold; λ₀ = 0 as the generative zero where all dualities meet.
- `[[feedback-no-bare-types]]` — always newtype; bare primitives let
  same-shape different-meaning values flow through. `fragment(altitude)`
  is a parametric type, not a record literal — the altitude is the
  newtype discipline.
- `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md` —
  `shift(T)` / `settle(T)` as type-layer operations; the FP-heritage
  map.
- `docs/insights/2026-05-26-mirror-sub-turing-substrate-with-emergent-turing-completeness.md`
  — the sub-Turing claim's operational form.
- `docs/insights/2026-05-26-kintsugi-as-credo-and-formatter-unified.md`
  — kintsugi as the one loop.

### 10.2 Mirror specs this spec rides on

- `docs/specs/mosaic.md` — the build-altitude reading; this spec
  generalizes it.
- `docs/specs/au-and-conductivity.md` — au as the output type of Fate
  inference; conductivity as verification.
- `docs/specs/transparency.md` — `transparency<p>` as the located loss
  carrier.
- `docs/specs/prism-core-as-spectral-triple.md` — the bootstrap as
  spectral-triple evaluator.
- `docs/specs/type-theory-position.md` — the literature survey this
  spec consolidates.
- `docs/specs/strict-and-total-classification.md` — the silence dies;
  dark fragments surface.
- `docs/specs/spectral-triple-binary.md` — the binary IS the spectral
  triple.
- `docs/specs/prism-floor-and-the-grammar-rename.md` — the substrate-
  pull rename (zoom → shift; refract → settle).
- `docs/specs/mirror-store.md` — the fragmentation store as canonical;
  `.shatter` as projection.
- `docs/specs/reality-shard-as-crdt.md` — SpectralUuid layout, monoid
  homomorphism, and the λ₀ = 0 substrate-first-address identification.
  The grounding for §4 of this spec.
- `docs/specs/mirror-spec-schema.md` — the project manifold grammar.
- `docs/specs/kintsugi-wiring.md` — the eight wires of the kintsugi
  loop.
- `docs/specs/the-convergence.md` — one runtime, four transports,
  five operations.
- `docs/specs/cli-as-prism.md` — the CLI as a prism with four glasses.
- `roadmap/wip/butterfly-self-hosting.md` — the runtime this type
  system projects into.

### 10.3 Academic literature

#### Total functional programming and termination

- Turner, D.A. (1995, 2004). *Elementary Strong Functional Programming*
  (Functional Programming Languages in Education) and *Total Functional
  Programming* (J.UCS 10(7), 2004).
  [PDF (Turner 1995)](https://www.cs.kent.ac.uk/people/staff/dat/esfp/fple.pdf),
  [nLab summary](https://ncatlab.org/ufias2012/files/turner.pdf).
- Wikipedia: [Total functional programming](https://en.wikipedia.org/wiki/Total_functional_programming).

#### Calculus of (Inductive) Constructions

- Coquand, T. & Huet, G. (1985). *The Calculus of Constructions*.
  HAL-Inria, [PDF](https://inria.hal.science/inria-00076024/PDF/RR-0530.pdf).
- Wikipedia: [Calculus of constructions](https://en.wikipedia.org/wiki/Calculus_of_constructions).
- nLab: [calculus of constructions](https://ncatlab.org/nlab/show/calculus+of+constructions).

#### Lambda calculus, System F, parametric polymorphism

- Church, A. (1936). *An unsolvable problem of elementary number
  theory*. American Journal of Mathematics 58.
- Girard, J.-Y. (1972). *Interprétation fonctionnelle et élimination
  des coupures de l'arithmétique d'ordre supérieur*. PhD thesis,
  Paris VII.
- Reynolds, J.C. (1974). *Towards a theory of type structure*.
  Colloque sur la Programmation.

#### Dependent type theory

- Martin-Löf, P. (1972). *An intuitionistic theory of types*.
  Manuscript; published in 1998 as Twenty-Five Years of Constructive
  Type Theory.
- Brady, E. (2013). *Type-Driven Development with Idris*. Manning.
- The Univalent Foundations Program (2013). *Homotopy Type Theory:
  Univalent Foundations of Mathematics*.

#### Category theory and optics

- Mac Lane, S. (1971). *Categories for the Working Mathematician*.
  Springer.
- Eilenberg, S. & Mac Lane, S. (1945). *General theory of natural
  equivalences*. Transactions of the AMS.
- Clarke, B., Elkins, D., Gibbons, J., Sherrell, F., Sherrill, L.,
  & Van der Ploeg, A. (2020). *Profunctor Optics: a Categorical Update*.
  Compositionality.
- Milewski, B. (2017). *Profunctor Optics: The Categorical View*.

#### Curry-Howard correspondence

- Curry, H.B. (1934). *Functionality in combinatory logic*.
  Proceedings of the National Academy of Sciences 20.
- Howard, W.A. (1969). *The formulae-as-types notion of construction*.
  Published 1980 in *To H. B. Curry: Essays on Combinatory Logic,
  Lambda Calculus and Formalism*.
- Wikipedia: [Curry–Howard correspondence](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence).

#### Connes noncommutative geometry

- Connes, A. (1994). *Noncommutative Geometry*. Academic Press.
- Connes, A. (1996). *Gravity coupled with matter and the foundation
  of non-commutative geometry*. Communications in Mathematical Physics
  182, 155–176.
- Connes, A. & Chamseddine, A.H. (1997). *The Spectral Action Principle*.
  Communications in Mathematical Physics.
- Connes, A. (2019). *Noncommutative Geometry, the Spectral Standpoint*.
  arXiv:1910.10407, [PDF](https://alainconnes.org/wp-content/uploads/NCGspectral.pdf).
- van Suijlekom, W. (2014). *Noncommutative Geometry and Particle
  Physics*. Springer.
- Dąbrowski, L., D'Andrea, F., & Sitarz, A. (2023). *A Critical Survey
  of Twisted Spectral Triples Beyond the Standard Model*.
  arXiv:2301.08346.

#### Content-addressed languages

- The Unison Computing team. *Unison: The big idea*.
  [Documentation](https://www.unison-lang.org/docs/the-big-idea/).
- IPLD specifications. *Merkle DAGs and content-addressed data*.
  [ipld.io](https://ipld.io/), [ProtoSchool tutorial](https://proto.school/merkle-dags/08/).
- NixOS Wiki. *Ca-derivations (Floating content-addressed derivations)*.
  [wiki.nixos.org/wiki/Ca-derivations](https://wiki.nixos.org/wiki/Ca-derivations).

#### Linear/affine types and Quantitative Type Theory

- Wadler, P. (1990). *Linear types can change the world!* Programming
  Concepts and Methods, North-Holland.
- Atkey, R. (2018). *Syntax and Semantics of Quantitative Type Theory*.
  LICS.
- Orchard, D., Liepelt, V., & Eades III, H. (2019). *Quantitative
  Program Reasoning with Graded Modal Types*. ICFP.

#### Propagator networks

- Radul, A. & Sussman, G.J. (2009). *The Art of the Propagator*.
  MIT CSAIL Technical Report.
  [groups.csail.mit.edu](https://groups.csail.mit.edu/mac/users/gjs/propagators/),
  [Semantic Scholar](https://www.semanticscholar.org/paper/The-Art-of-the-Propagator-Sussman-Radul/03552d63bdad10cb277e84e729fc2a556c3bf8f0).

#### Typed holes and live programming

- Omar, C., Voysey, I., Hilton, M., Aldrich, J., & Hammer, M. (2017).
  *Hazelnut: A Bidirectionally Typed Structure Editor Calculus*. POPL.
- Omar, C., Voysey, I., Chugh, R., & Hammer, M. (2019). *Live
  Functional Programming with Typed Holes*. POPL.

#### Spectral triples and contextuality

- Magnot, J.-P. (2025). *Contextuality, Holonomy and Discrete Fiber
  Bundles in Group-Valued Boltzmann Machines*. arXiv:2509.10536.
- Hansen, J. & Ghrist, R. (2019). *Toward a Spectral Theory of
  Cellular Sheaves*. Journal of Applied and Computational Topology.

#### Void / lambda-zero references

- Braunstein, S., Ghosh, S., & Severini, S. (2006). *The Laplacian of
  a Graph as a Density Matrix*. arXiv:quant-ph/0406165.
- Passerini, F. & Severini, S. (2008). *The Von Neumann Entropy of
  Networks*. arXiv:0812.2597.
- Soto-Andrade, J. & Varela, F.J. (1984). *Self-reference and
  fixed-point theorems*. Acta Applicandae Mathematicae 2:1.

---

*Mosaic is the algebra. Splinters are the atoms it acts on.*
*Five operations generate the algebra. Three shapes close the splinter.*
*au is Fate-emitted splinters. Shard is what the store keeps.*
*Settle composes splinters into shards — the Kleisli arrow in Imperfect.*
*SpectralUuid identifies the shard. Active bits navigate;*
*dark bits identify. Connes distance becomes computable.*
*Kintsugi descends the residual. eⁿ⁺¹ ≤ eⁿ at every altitude.*
*Bottom (splinter) and top (SpectralUuid) meet in the middle (shard).*
*The substrate has been pointing here since the void document.*
*This spec names where the pointing arrives — and where it still has to go.*

Apache-2.0.
